// A C++ program that prints a message to the terminal, from C++ and Rust

#include <iostream>

// Declare the Rust function so C++ can call it
extern "C" void print_rust_message(void);

// Prints a run message to the terminal, from C++ and Rust
int main(void) {
    // Replace this with your C++ code
    std::cout << "C++ code ran while running the build-tool-template example" << std::endl;
    print_rust_message();
    return 0;
}
