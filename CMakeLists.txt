cmake_minimum_required(VERSION 3.10)

project("call-rust-from-fortran" LANGUAGES Fortran CXX)

add_executable(call_rust "src/hello.f90")
add_executable(call_rust_from_c "src/hello.cpp")

include("cmake/rust.cmake")
link_rust_library(call_rust)
link_rust_library(call_rust_from_c)