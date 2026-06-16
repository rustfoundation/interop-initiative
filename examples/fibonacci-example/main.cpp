// A C++ program that calls a Rust fibonacci function via FFI
#include <iostream>

// Mirror the Rust FibResult struct in C++
struct FibResult {
    unsigned long long value;
    unsigned char overflow;
};

// Declare the Rust function so C++ can call it
extern "C" FibResult fibonacci(unsigned int n);

int main(void) {
    for (unsigned int i = 0; i <= 10; i++) {
        FibResult result = fibonacci(i);
        if (result.overflow) {
            std::cout << "fibonacci(" << i << ") = overflow!" << std::endl;
        } else {
            std::cout << "fibonacci(" << i << ") = " << result.value << std::endl;
        }
    }

    // Test overflow case
    FibResult overflow_test = fibonacci(100);
    std::cout << "fibonacci(100) overflow flag = " 
              << (int)overflow_test.overflow << std::endl;

    return 0;
}
