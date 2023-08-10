program hello_world
    ! We need to use the ISO_C_BINDING module to declare the types compatible with C
    use iso_c_binding

    implicit none

    ! Declare a type that corresponds to the Rust struct.
    type, bind(C) :: rustacean_struct
        integer(c_int) :: number
        real(c_float) :: pi
    end type

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

        ! This is the same as the previous one, but we declare the argument as reference instead of value.
        subroutine display(s) BIND(C)
            use iso_c_binding
            import :: rustacean_struct
            type(rustacean_struct), intent(in) :: s ! Note: no VALUE attribute
        end subroutine
    end interface

    type(rustacean_struct) :: s
    s%number = 42
    s%pi = 3.14

    print *, "Hello from Fortran!"
    call hello_from_rust()
    print *, "Sum of 2 and 3 is", rustacean_sum(2, 3)
    call display(s)

end program hello_world