#[[
This file provides the "link_rust_library" function, which can be used to link
the Rust library to a target.
]]

# This function detects the build type and sets the Rust build type variable accordingly
#
# You can modify this function by adding your own build types, but remember to add a
# corresponding rust profile in the rust/Cargo.toml file.
function(detect_rust_build_type _rust_build_type)
    if(CMAKE_BUILD_TYPE STREQUAL "Debug" OR NOT CMAKE_BUILD_TYPE)
        set(${_rust_build_type} "debug" PARENT_SCOPE)
    elseif(CMAKE_BUILD_TYPE STREQUAL "Release")
        set(${_rust_build_type} "release" PARENT_SCOPE)
    elseif(CMAKE_BUILD_TYPE STREQUAL "RelWithDebInfo")
        set(${_rust_build_type} "rel-with-deb-info" PARENT_SCOPE)
    elseif(CMAKE_BUILD_TYPE STREQUAL "MinSizeRel")
        set(${_rust_build_type} "min-size-rel" PARENT_SCOPE)
    else()
        message(FATAL_ERROR "Unknown build type: ${CMAKE_BUILD_TYPE}")
    endif()
endfunction()

# This function detects the suffixes for dynamic and static libraries according to the
# current platform.
#
# You can modify this function by adding platform-specific suffixes.
function(detect_profile_suffixes _prefix _dyn_lib_suffix _lib_suffix)
    if(WIN32)
        set(${_prefix} "" PARENT_SCOPE)
        set(${_dyn_lib_suffix} ".dll" PARENT_SCOPE)
        set(${_lib_suffix} ".dll.lib" PARENT_SCOPE)
    elseif(UNIX)
        set(${_prefix} "lib" PARENT_SCOPE)
        set(${_dyn_lib_suffix} ".so" PARENT_SCOPE)
        set(${_lib_suffix} ".so" PARENT_SCOPE)
    elseif(APPLE)
        set(${_prefix} "lib" PARENT_SCOPE)
        set(${_dyn_lib_suffix} ".dylib" PARENT_SCOPE)
        set(${_lib_suffix} ".a" PARENT_SCOPE)
    else()
        message(FATAL_ERROR "Unknown platform")
    endif()
endfunction()

# target that builds the Rust library
detect_rust_build_type(rust_build_type)
set(cargo_dir "${CMAKE_CURRENT_LIST_DIR}/../rust")
if (rust_build_type STREQUAL "debug")
    add_custom_target(
        build_rust_lib
        COMMAND cargo build
        WORKING_DIRECTORY ${cargo_dir}
    )
else()
    add_custom_target(
        build_rust_lib
        COMMAND cargo build --profile ${rust_build_type}
        WORKING_DIRECTORY ${cargo_dir}
    )
endif()

# links a target to the Rust library
function(link_rust_library target)
    detect_rust_build_type(rust_build_type rust_profile_args)
    detect_profile_suffixes(lib_prefix dyn_lib_suffix lib_suffix)

    # link the Rust library
    target_link_libraries(
        ${target}
        PRIVATE
        "${CMAKE_CURRENT_LIST_DIR}/rust/target/${rust_build_type}/${lib_prefix}rust${lib_suffix}"
    )

    # add the Rust library include directory (not needded for Fortran)
    target_include_directories(${target} PRIVATE "${CMAKE_CURRENT_LIST_DIR}/rust/bindings")

    # make sure the Rust library is built before the target
    add_dependencies(${target} build_rust_lib)

    # copy the Rust library to the target directory after building
    add_custom_command(
        TARGET ${target} POST_BUILD
        COMMAND
            ${CMAKE_COMMAND} -E copy_if_different
            "${CMAKE_CURRENT_LIST_DIR}/rust/target/${rust_build_type}/${lib_prefix}rust${dyn_lib_suffix}"
            $<TARGET_FILE_DIR:${target}>
    )
endfunction()