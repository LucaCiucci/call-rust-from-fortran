module rust
    use iso_c_binding
    implicit none

    !Some Rustacean struct
    type, bind(C) :: rustacean_struct
        integer(c_int) :: number
        real(c_float) :: pi
        integer(c_int) :: e
    end type rustacean_struct

    enum, bind(C)
        ! enum SomeEnum:
        enumerator :: SomeEnum_A
        enumerator :: SomeEnum_B
    end enum

    interface
        ! Greet from Rust
        subroutine hello_from_rust() bind(C, name="hello_from_rust")
            use iso_c_binding
        end subroutine hello_from_rust

        ! Sum two numbers
        function rustacean_sum(a, b) result(result__) bind(C, name="rustacean_sum")
            use iso_c_binding
            integer(c_int), value, intent(in) :: a
            integer(c_int), value, intent(in) :: b
            integer(c_int) :: result__
        end function rustacean_sum

        ! Same as [rustacean_sum] but with pointers
        subroutine display_rustacean_struct(s) bind(C, name="display_rustacean_struct")
            use iso_c_binding
            import :: rustacean_struct
            type(rustacean_struct), intent(in) :: s
        end subroutine display_rustacean_struct

    end interface
end module rust
