#include <iostream>

// Declare Rust functions using C ABI
extern "C" void trigger_panic();
extern "C" void trigger_panic_safe();

int main() {
    std::cout << "=== Calling SAFE Rust function ===" << std::endl;

    // SAFE CASE:
    // The panic is handled inside Rust using catch_unwind.
    // This should NOT crash the program.
    trigger_panic_safe();

    std::cout << "\n=== Calling UNSAFE Rust function ===" << std::endl;

    // UNSAFE CASE:
    // This triggers a panic inside an extern "C" function.
    // Rust will abort the program to prevent undefined behavior.
    trigger_panic();

    // This line will likely never execute
    std::cout << "This line will not be reached." << std::endl;

    return 0;
}