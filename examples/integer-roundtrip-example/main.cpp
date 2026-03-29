// A C++ program that demonstrates lossless integer roundtrips between Rust and C++
#include <iostream>

// Declare the Rust function so C++ can call it
extern "C" unsigned int roundtrip_integer(unsigned int value);

// Demonstrates sending an integer to Rust and receiving it back losslessly
int main(void) {
    unsigned int value = 4294967295; // u32::MAX - would truncate if handled incorrectly
    // Send integer to Rust and receive it back
    std::cout << "C++ sending value: " << value << std::endl;
    
    unsigned int result = roundtrip_integer(value);
    
    std::cout << "C++ received back: " << result << std::endl;
    
    // Verify the roundtrip was lossless
    if (value == result) {
        std::cout << "Roundtrip was lossless!" << std::endl;
    }
    
    return 0;
}
