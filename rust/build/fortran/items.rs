use cbindgen::{Bindings, ItemContainer, Struct};

use super::functions::declaration_type;



pub fn write_items(
    file: &mut dyn std::io::Write,
    bindings: &Bindings,
) -> std::io::Result<()> {
    for item in bindings.items() {
        match item {
            ItemContainer::Constant(_) => todo!(),
            ItemContainer::Static(_) => todo!(),
            ItemContainer::OpaqueItem(_) => todo!(),
            ItemContainer::Struct(s) => write_struct(file, s, bindings)?,
            ItemContainer::Union(_) => todo!(),
            ItemContainer::Enum(_) => todo!(),
            ItemContainer::Typedef(_) => todo!(),
        }
    }

    Ok(())
}

fn write_struct(
    file: &mut dyn std::io::Write,
    s: &Struct,
    bindings: &Bindings,
) -> std::io::Result<()> {
    let name = &s.export_name;

    for comment in &s.documentation.doc_comment {
        writeln!(file, "    ! {}", comment.trim())?;
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