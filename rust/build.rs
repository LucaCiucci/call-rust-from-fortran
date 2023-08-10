// see https://github.com/mozilla/cbindgen/blob/v0.24.3/docs.md

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .with_header("// Generated by cbindgen, DO NOT EDIT")
      //.with_language(cbindgen::Language::C)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("bindings/rust.hpp");
}