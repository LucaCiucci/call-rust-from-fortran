use cbindgen::{Bindings, ir::{ItemContainer, Struct, Item}};

use self::enums::{EnumsWriter, Variant};

use super::functions::declaration_type;

pub mod enums;

pub fn write_items(
    file: &mut dyn std::io::Write,
    bindings: &Bindings,
) -> std::io::Result<()> {

    let mut enums = EnumsWriter::new();
    
    for item in bindings.items() {
        match item {
            ItemContainer::Constant(_) => todo!(),
            ItemContainer::Static(_) => todo!(),
            ItemContainer::OpaqueItem(_) => todo!(),
            ItemContainer::Struct(s) => write_struct(file, s, bindings)?,
            ItemContainer::Union(_) => todo!(),
            ItemContainer::Enum(e) => {
                enums.add_enum(e.name());
                for variant in &e.variants {
                    enums.variant(Variant {
                        name: variant.name.clone(),
                        docs: variant.documentation.doc_comment.clone(),
                    })
                }
            },
            ItemContainer::Typedef(_) => todo!(),
        }
    }
    writeln!(file)?;

    enums.write(file)?;


    Ok(())
}

fn write_struct(
    file: &mut dyn std::io::Write,
    s: &Struct,
    bindings: &Bindings,
) -> std::io::Result<()> {
    let name = &s.export_name;

    for comment in &s.documentation.doc_comment {
        writeln!(file, "    !{}", comment.trim())?;
    }

    writeln!(file, "    type, bind(C) :: {name}")?;

    for field in &s.fields {
        for comment in &field.documentation.doc_comment {
            writeln!(file, "        ! {}", comment)?;
        }

        writeln!(file, "        {} :: {}", declaration_type(&field.ty, bindings), field.name)?;
    }

    writeln!(file, "    end type {name}")?;

    Ok(())
}