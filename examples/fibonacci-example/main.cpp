// A C++ program that calls a Rust fibonacci function via FFI
#include <iostream>

// Declare the Rust function so C++ can call it
extern "C" unsigned long long fibonacci(unsigned int n);

// Prints fibonacci numbers from 0 to 10
int main(void) {
    for (unsigned int i = 0; i <= 10; i++) {
        std::cout << "fibonacci(" << i << ") = " 
                  << fibonacci(i) << std::endl;
    }
    return 0;
}
