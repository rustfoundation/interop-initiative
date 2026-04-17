#include <iostream>
#include "rust_function.h" //This is the  header that declares the Rust functions we're calling

int main() {
    int result = add(3, 4);  // I am calling the Rust add() across the FFI boundary 
    std::cout << "3 + 4 = " << result << std::endl;
    return 0;
}