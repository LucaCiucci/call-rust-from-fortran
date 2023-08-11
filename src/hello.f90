program hello_world
    ! We need to use the ISO_C_BINDING module to declare the types compatible with C
    use iso_c_binding
    use rust
    use some_module

    implicit none

    type(rustacean_struct) :: s
    s%number = 42
    s%pi = 3.14

    print *, "Hello from Fortran!"
    call hello_from_rust()
    print *, "Sum of 2 and 3 is", rustacean_sum(2, 3)
    call display(s)
    call hello()

end program hello_world