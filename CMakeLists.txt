cmake_minimum_required(VERSION 3.10)
project("call-rust-from-fortran" LANGUAGES Fortran CXX)

# define the two example targets: the Fortran and C++ executables
add_executable(call_rust "src/hello.f90")
add_executable(call_rust_from_c "src/hello.cpp")

# link the rust library to the executables
include("cmake/rust.cmake")
link_rust_library(call_rust)
link_rust_library(call_rust_from_c)