cmake_minimum_required(VERSION 3.10)

project("call-rust-from-fortran" LANGUAGES "Fortran")
add_executable(call_rust "src/hello.f90")

# detect build type and set the Rust build type variable accordingly
# we will use this variable to build the Rust library with the same build type
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(rust_build_type "release")
elseif(CMAKE_BUILD_TYPE STREQUAL "Release")
    set(rust_build_type "release")
elseif(CMAKE_BUILD_TYPE STREQUAL "RelWithDebInfo")
    set(rust_build_type "rel-with-deb-info")
elseif(CMAKE_BUILD_TYPE STREQUAL "MinSizeRel")
    set(rust_build_type "min-size-rel")
else()
    message(FATAL_ERROR "Unknown build type: ${CMAKE_BUILD_TYPE}")
endif()

# detect platform
if(WIN32)
    set(dyn_lib_suffix "dll")
    set(lib_suffix "lib")
elseif(UNIX)
    set(dyn_lib_suffix "so")
    set(lib_suffix "a")
elseif(APPLE)
    set(dyn_lib_suffix "dylib")
    set(lib_suffix "a")
else()
    message(FATAL_ERROR "Unknown platform")
endif()

# before building the Fortran program, build the Rust library
add_custom_target(
    build_rust_lib
    COMMAND cargo build --profile ${rust_build_type}
    WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/rust"
)
add_dependencies(call_rust build_rust_lib)

# link the Rust library to the Fortran program
target_link_libraries(
    call_rust
    PRIVATE
    "${CMAKE_CURRENT_SOURCE_DIR}/rust/target/${rust_build_type}/rust.${dyn_lib_suffix}.${lib_suffix}"
)

# after building the Fortran program, copy the Rust library to the same directory
add_custom_command(
    TARGET call_rust POST_BUILD
    COMMAND
        ${CMAKE_COMMAND} -E copy_if_different
        "${CMAKE_CURRENT_SOURCE_DIR}/rust/target/${rust_build_type}/rust.${dyn_lib_suffix}"
        $<TARGET_FILE_DIR:call_rust>
)