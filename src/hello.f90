program hello_world
    ! We need to use the ISO_C_BINDING module to declare the types compatible with C
    use iso_c_binding
    use rust
    use some_module

    implicit none

    type(rustacean_struct) :: s
    s%number = 42
    s%pi = 3.14
    s%e = SomeEnum_A

    print *, "Hello from Fortran!"
    call hello_from_rust()
    print *, "2 + 3 = ", rustacean_sum(2, 3)
    call display_rustacean_struct(s)
    call hello()

end program hello_world