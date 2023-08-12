use std::collections::HashSet;



pub struct EnumsWriter {
    items: Vec<Item>,
    variant_names: HashSet<String>,
}

pub struct Variant {
    pub name: String,
    pub docs: Vec<String>,
}

enum Item {
    Variant(Variant),
    Enum(String),
}

impl EnumsWriter {
    pub fn new() -> Self {
        EnumsWriter {
            items: Vec::new(),
            variant_names: HashSet::new(),
        }
    }

    pub fn add_enum(&mut self, name: &str) {
        self.items.push(Item::Enum(name.to_string()));
    }

    pub fn variant(&mut self, variant: Variant) {
        if self.variant_names.contains(&variant.name) {
            panic!("Duplicate variant name: {}", variant.name);
        }
        self.variant_names.insert(variant.name.clone());
        self.items.push(Item::Variant(variant));
    }

    pub fn write(&self, file: &mut dyn std::io::Write) -> std::io::Result<()> {
        writeln!(file, "    enum, bind(C)")?;

        for item in &self.items {
            match item {
                Item::Variant(variant) => {
                    for comment in &variant.docs {
                        writeln!(file, "        ! {}:", comment.trim())?;
                    }
                    writeln!(file, "        enumerator :: {}", variant.name)?;
                }
                Item::Enum(name) => writeln!(file, "        ! enum {}:", name)?,
            }
        }

        writeln!(file, "    end enum")?;

        Ok(())
    }
}