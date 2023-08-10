program hello_world
    implicit none

    ! We place the declarations of the Rust functions in an interface block
    !
    ! See https://gcc.gnu.org/onlinedocs/gcc-4.9.0/gfortran/Interoperability-with-C.html for more details.
    interface
        ! This is a function with no arguments and no return value.
        ! Note that we declare it as a SUBROUTINE, not a FUNCTION because it has no return value.
        ! Also note that we need to specify the BIND(C) attribute to tell the compiler to use the C calling convention and not add any name mangling.
        subroutine hello_from_rust() BIND(C)
        end subroutine

        ! This is a function with two arguments and a return value.
        ! It has a return value, so we declare it as a FUNCTION.
        function rustacean_sum(a, b) result(c) BIND(C)
            ! We need to use the ISO_C_BINDING module to declare the types of the arguments and return value.
            use iso_c_binding

            ! The input arguments.
            ! Notes:
            ! - we have to use the c_int type from the ISO_C_BINDING module
            !   to guarantee that it is the same size and encoding as a C int.
            ! - we have to specify the INTENT(IN) attribute to tell the compiler that these are input arguments.
            ! - we have to specify the VALUE attribute to tell the compiler that these are passed by value and not by reference,
            !   see https://gcc.gnu.org/onlinedocs/gcc-4.9.0/gfortran/Interoperable-Subroutines-and-Functions.html#Interoperable-Subroutines-and-Functions
            !   for more details.
            integer(c_int), intent(in), value :: a, b

            ! The return value.
            integer(c_int) :: c
        end function

        ! This is the same as the previous function, but we declare the arguments as references instead of values.
        function rustacean_sum_from_ptrs(a, b) result(c) BIND(C)
            use iso_c_binding
            integer(c_int), intent(in) :: a, b ! Note: no VALUE attribute
            integer(c_int) :: c
        end function
    end interface

    print *, "Hello from Fortran!"
    call hello_from_rust()
    print *, "Sum of 2 and 3 is", rustacean_sum(2, 3)
    print *, "Sum of 2 and 3 is", rustacean_sum_from_ptrs(2, 3)

end program hello_world