#include <iostream>

extern "C" {
    char* get_message();
    void print_message(const char* msg);
    void free_string(char* msg);
}

int main() {
    // Get string from Rust
    char* msg = get_message();

    std::cout << "C++ received: " << msg << std::endl;

    // Send it back to Rust
    print_message(msg);

    // Free memory
    free_string(msg);

    return 0;
}