// A C++ program that calls Rust math functions
//
// This demonstrates adding Rust to an existing C/C++ build system,
// where C++ owns the main function and the build process.

#include <iostream>

// Declare the Rust functions so C++ can call them
extern "C" int32_t add(int32_t a, int32_t b);
extern "C" int32_t multiply(int32_t a, int32_t b);

int main(void) {
    std::cout << "Calling Rust functions from C++:" << std::endl;
    std::cout << "  add(3, 4) = " << add(3, 4) << std::endl;
    std::cout << "  multiply(3, 4) = " << multiply(3, 4) << std::endl;
    return 0;
}
