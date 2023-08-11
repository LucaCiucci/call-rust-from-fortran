module rust
    use iso_c_binding
    implicit none

    ! Some Rustacean struct
    type, bind(C) :: rustacean_struct
        integer(c_int) :: number
        real(c_float) :: pi
    end type rustacean_struct

    interface
        !  Greet from Rust
        subroutine hello_from_rust() bind(C, name="hello_from_rust")
            use iso_c_binding
!0
        end subroutine hello_from_rust

        !  Sum two numbers
        function rustacean_sum(a, b) result(result__) bind(C, name="rustacean_sum")
            use iso_c_binding
!0
            integer(c_int), value, intent(in) :: a
            integer(c_int), value, intent(in) :: b
            integer(c_int) :: result__
        end function rustacean_sum

        !  Same as [rustacean_sum] but with pointers
        subroutine display(s) bind(C, name="display")
            use iso_c_binding
!1
            import :: rustacean_struct
            type(rustacean_struct), intent(in) :: s
        end subroutine display

    end interface
end module rust
