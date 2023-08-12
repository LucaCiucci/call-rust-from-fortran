use std::collections::HashSet;

use cbindgen::{Bindings, bindgen::ir::{Type, FunctionArgument, PrimitiveType, IntKind}, ir::{GenericPath, ItemContainer}};



pub fn write_functions(
    file: &mut dyn std::io::Write,
    bindings: &Bindings,
) -> std::io::Result<()> {
    for function in bindings.functions() {
        let ret = &function.ret;
        let returns_void = function.never_return || match ret {
            Type::Primitive(PrimitiveType::Void) => true,
            _ => false,
        };
        let name = function.path().name();

        let mut arg_counter = 0;

        let arg_names = function
            .args
            .iter()
            .map(|arg| {
              match &arg.name {
                  Some(name) => name.clone(),
                  None => {
                      arg_counter += 1;
                      format!("arg_{}", arg_counter)
                  }
              }
            })
            .collect::<Vec<_>>();
        
        let args_list = arg_names.join(", ");

        let mut imports = HashSet::new();

        for arg in &function.args {
            if let Type::Path(path) = &arg.ty {
                imports.insert(path.name());
            }
            if let Type::Ptr { ty, .. } = &arg.ty {
                if let Type::Path(path) = &**ty {
                    imports.insert(path.name());
                }
            }
        }
        if let Type::Path(path) = &ret {
            imports.insert(path.name());
        }

        for comment in &function.documentation.doc_comment {
            writeln!(file, "        !{}", comment)?;
        }

        if returns_void {
            writeln!(file, "        subroutine {}({args_list}) bind(C, name=\"{}\")", name, name)?;
        } else {
            writeln!(file, "        function {}({args_list}) result(result__) bind(C, name=\"{}\")", name, name)?;
        }

        //writeln!(file, "            implicit none")?;
        writeln!(file, "            use iso_c_binding")?;

        for import in imports {
            writeln!(file, "            import :: {}", import)?;
        }

        for i in 0..arg_names.len() {
            let arg = &function.args[i];
            writeln!(file, "            {}", declare_argument_type(arg, &arg_names[i], bindings))?;
        }

        if returns_void {
            writeln!(file, "        end subroutine {}", name)?;
        } else {
            writeln!(file, "            {}", declare_return_type(ret, bindings))?;
            writeln!(file, "        end function {}", name)?;
        }
        writeln!(file)?;
    }

    Ok(())
}

fn declare_argument_type(arg: &FunctionArgument, name: &str, bindings: &Bindings) -> String {
    let ty = argument_declaration_type(&arg.ty, bindings);
    format!("{}, intent(in) :: {}", ty, name)
}

fn declare_return_type(ty: &Type, bindings: &Bindings) -> String {
    let ty = declaration_type(ty, bindings);
    format!("{} :: result__", ty)
}

fn find_item<'a>(path: &GenericPath, bindings: &'a Bindings) -> Option<&'a ItemContainer> {
    bindings
        .items()
        .iter()
        .find(|item| {
            item.deref().path().name() == path.name()
        })
}

fn argument_declaration_type(ty: &Type, bindings: &Bindings) -> String {
    match ty {
        Type::Ptr { ty, .. } => declaration_type(&ty, bindings),
        Type::Path(path) => {
            if let Some(item) = find_item(path, bindings) {
                match item {
                    ItemContainer::Enum(_) => format!("integer(c_int)"),
                    _ => format!("type({})", path.name())
                }
            } else {
                unreachable!()
            }
        },
        Type::Primitive(ty) => format!("{}, value", primitive_declaration_type(ty)),
        Type::Array(_, _) => todo!(),
        Type::FuncPtr { .. } => todo!(),
    }
}

pub fn declaration_type(ty: &Type, bindings: &Bindings) -> String {
    match ty {
        Type::Ptr { ty, .. } => declaration_type(&ty, bindings),
        Type::Path(path) => {
            if let Some(item) = find_item(path, bindings) {
                match item {
                    ItemContainer::Enum(_) => format!("integer(c_int)"),
                    _ => format!("type({})", path.name())
                }
            } else {
                panic!("Could not find item for path: {:?}", path);
                //unreachable!()
            }
        },
        Type::Primitive(ty) => format!("{}", primitive_declaration_type(ty)),
        Type::Array(_, _) => todo!(),
        Type::FuncPtr { .. } => todo!(),
    }
}

fn primitive_declaration_type(ty: &PrimitiveType) -> String {
    match ty {
      PrimitiveType::Void => unreachable!(), // TODO ???
      PrimitiveType::Bool => "logical".to_string(), // TODO ???
      PrimitiveType::Char => "character(c_char)".to_string(), // TODO ???
      PrimitiveType::SChar => "integer(c_char)".to_string(), // TODO ???
      PrimitiveType::UChar => "integer(c_char)".to_string(), // TODO ???
      PrimitiveType::Char32 => unreachable!(), // TODO ???
      PrimitiveType::Float => "real(c_float)".to_string(),
      PrimitiveType::Double => "real(c_double)".to_string(),
      PrimitiveType::VaList => todo!(),
      PrimitiveType::PtrDiffT => todo!(),
      PrimitiveType::Integer { kind, .. } => {
          match kind {
              IntKind::Short => "integer(c_short)".to_string(),
              IntKind::Int => "integer(c_int)".to_string(),
              IntKind::Long => "integer(c_long)".to_string(),
              IntKind::LongLong => "integer(c_long_long)".to_string(),
              IntKind::SizeT => "integer(c_size_t)".to_string(),
              IntKind::Size => "integer(c_size_t)".to_string(),
              IntKind::B8 => "integer(c_int8_t)".to_string(),
              IntKind::B16 => "integer(c_int16_t)".to_string(),
              IntKind::B32 => "integer(c_int32_t)".to_string(),
              IntKind::B64 => "integer(c_int64_t)".to_string(),
          }
      }
    }
}