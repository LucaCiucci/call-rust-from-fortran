use cbindgen::CustomLanguageBindgen;

use self::{functions::write_functions, items::write_items};

mod functions;
mod items;

#[derive(Debug)]
pub struct Fortran(pub String);
impl CustomLanguageBindgen for Fortran {
    fn write(
        &self,
        bindings: &cbindgen::Bindings,
        file: &mut dyn std::io::Write,
        _config: &cbindgen::Config
    ) -> std::io::Result<()> {
        writeln!(file, "module {}", self.0)?;
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

        writeln!(file, "end module {}", self.0)?;

        Ok(())
    }
}