#include <iostream>

// Tell C++ that fibonacci function exists in Rust
extern "C" unsigned long long fibonacci(unsigned int n);

int main() {
    // Call Rust fibonacci function from C++
    for (int i = 0; i <= 10; i++) {
        std::cout << "fibonacci(" << i << ") = " 
                  << fibonacci(i) << std::endl;
    }
    return 0;
}
