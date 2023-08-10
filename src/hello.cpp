
#include <iostream>

#include <rust.hpp>

int main(int argc, char* argv[])
{
    std::cout << "Hello, World!" << std::endl;
    hello_from_rust();
    std::cout << "2 + 3 = " << rustacean_sum(2, 3) << std::endl;

    return 0;
}