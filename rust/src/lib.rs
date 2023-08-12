use std::ffi::{c_int, c_float};

/// Greet from Rust
#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
}

/// Sum two numbers
#[no_mangle]
pub extern "C" fn rustacean_sum(a: c_int, b: c_int) -> c_int {
    a + b
}

/// Some Rustacean struct
#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct rustacean_struct {
    pub number: c_int,
    pub pi: c_float,
    pub e: SomeEnum,
}

/// Same as [rustacean_sum] but with pointers
#[no_mangle]
pub extern "C" fn display_rustacean_struct(s: &rustacean_struct) {
    println!("{:#?}", s);
}

#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum SomeEnum {
    SomeEnum_A,
    SomeEnum_B,
}