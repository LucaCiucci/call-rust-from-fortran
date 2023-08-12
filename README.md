# Call Rust from Fortran

This is an example project that shows how to call Rust from Fortran.

Project structure:
 - [`src`](./src/) contains the Fortran source code and the equivalent C source code
 - [`rust`](./rust/) contains the Rust source code
 - [`CMakeLists.txt`](./CMakeLists.txt) is the CMake file that builds the project
 - [`cmake/rust.cmake`](./cmake/rust.cmake) tells CMake how to build and link the Rust code

## Build

To build the project, run the following commands:
```sh
mkdir build
cd build
cmake ..
cmake --build .
```
or just click your IDE's _build_ button.

## Additional Material

- [Interoperability with C - GCC](https://gcc.gnu.org/onlinedocs/gcc-4.9.0/gfortran/Interoperability-with-C.html)

## Call Fortran from Rust

See <https://github.com/harbik/dierckx-sys>