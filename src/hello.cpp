/*
This is the C++ equivalent of the file hello.f90
*/


#include <iostream>

#include <rust.hpp>

constexpr int a = 2;
constexpr int b = 3;

int main(int argc, char* argv[])
{
    std::cout << "Hello from C++!" << std::endl;
    hello_from_rust();
    std::cout << a << " + " << b << " = " << rustacean_sum(a, b) << std::endl;
    
    const rustacean_struct s = {
        42,
        3.14,
        SomeEnum::SomeEnum_A,
    };
    display_rustacean_struct(&s);
    return 0;
}