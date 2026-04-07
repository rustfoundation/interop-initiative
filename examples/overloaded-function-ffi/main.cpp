#include <iostream>

// Two overloaded C++ functions with the same name but different types
int add(int a, int b) {
    return a + b;
}

double add(double a, double b) {
    return a + b;
}

// Exposing a simple Rust function via FFI.
// While Rust supports generics and traits for abstraction,
// FFI requires concrete function signatures with fixed symbol names.
extern "C" int add_int(int a, int b) {
    return add(a, b);
}

extern "C" double add_double(double a, double b) {
    return add(a, b);
}

// // Rust functions exposed via FFI
extern "C" int call_add_int(int a, int b);
extern "C" double call_add_double(double a, double b);

int main() {
    std::cout << "Calling overloaded C++ functions from Rust.." << std::endl;

    // Call Rust functions, which internally call the C++ wrappers
    std::cout << "add_int(2, 3) = " << call_add_int(2, 3) << std::endl;
    std::cout << "add_double(2.5, 3.5) = " << call_add_double(2.5, 3.5) << std::endl;

    return 0;
}