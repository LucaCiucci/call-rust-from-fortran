program hello_world
    implicit none

    interface
        subroutine hello_from_rust() BIND(C)
            use iso_c_binding
        end subroutine
        function rustacean_sum(a, b) result(c) BIND(C)
            use iso_c_binding
            integer(c_int), intent(in), value :: a, b
            integer(c_int) :: c
        end function
        function rustacean_sum_from_ptrs(a, b) result(c) BIND(C)
            use iso_c_binding
            integer(c_int), intent(in) :: a, b
            integer(c_int) :: c
        end function
    end interface

    print *, "Hello from Fortran!"
    call hello_from_rust()
    print *, "Sum of 2 and 3 is", rustacean_sum(2, 3)
    print *, "Sum of 2 and 3 is", rustacean_sum_from_ptrs(2, 3)

end program hello_world