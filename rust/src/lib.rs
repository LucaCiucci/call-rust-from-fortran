use std::ffi::c_int;

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
}

#[no_mangle]
pub extern "C" fn rustacean_sum(a: c_int, b: c_int) -> c_int {
    a + b
}

#[no_mangle]
pub extern "C" fn rustacean_sum_from_ptrs(a: &c_int, b: &c_int) -> c_int {
    a + b
}