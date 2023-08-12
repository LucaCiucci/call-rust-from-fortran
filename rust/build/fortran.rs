use cbindgen::CustomLanguage;

use self::{functions::write_functions, items::write_items};

mod functions;
mod items;

#[derive(Debug)]
pub struct Fortran {
    module_name: String,
}

impl Fortran {
    pub fn new(module_name: &str) -> Self {
        Self {
            module_name: module_name.to_string(),
        }
    }
}

impl CustomLanguage for Fortran {
    fn write(
        &self,
        file: &mut dyn std::io::Write,
        bindings: &cbindgen::Bindings,
        _config: &cbindgen::Config
    ) -> std::io::Result<()> {
        writeln!(file, "module {}", self.module_name)?;
        writeln!(file, "    use iso_c_binding")?;
        writeln!(file, "    implicit none")?;
        writeln!(file)?;

        for _constant in bindings.constants() {
            todo!()
        }

        write_items(file, bindings)?;
        writeln!(file)?;

        writeln!(file, "    interface")?;
        write_functions(file, bindings)?;
        writeln!(file, "    end interface")?;

        writeln!(file, "end module {}", self.module_name)?;

        Ok(())
    }
}