// This example demonstrates safe string interoperability between C++ and Rust,
// including ownership transfer and proper memory deallocation across the FFI boundary.

#include <iostream> // For input/output (std::cout, std::cin)
#include <string>   // For using std::string

// Declare Rust functions using C ABI so C++ can call them
extern "C" {
    void ask_name(); // Rust function to print prompt
    char* process_name(const char* name); // Rust function that processes input and returns a string
    void free_string(char* ptr); // Rust function to free allocated memory
}

int main() {
    // Step 1: Ask Rust to prompt user
    ask_name(); 
    // Calls Rust function which prints: "What is your name?"

    // Step 2: User input handled in C++
    std::string name; 
    // std::string stores user input safely in C++

    std::getline(std::cin, name); 
    // Read full line from terminal input into 'name'

    // Step 3: Send to Rust
    char* response = process_name(name.c_str()); 
    // Convert std::string → const char* using .c_str()
    // Pass pointer to Rust (FFI boundary)
    // Rust returns a new allocated string (ownership transferred to C++)

    // Step 4: Print response
    std::cout << "C++ received: " << response << std::endl; 
    // Print the string returned from Rust

    // Step 5: Free memory
    free_string(response); 
    // IMPORTANT: call Rust function to free memory
    // Prevents memory leak since Rust allocated it

    return 0; 
    // Program ends
}