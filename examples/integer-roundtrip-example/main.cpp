// Demonstrates lossless integer roundtrips between C++ and Rust
#include <iostream>

// Mirror the Rust RoundtripResult struct
struct RoundtripResult {
    unsigned int value;
    unsigned char was_lossy;
};

// Declare the Rust function
extern "C" RoundtripResult roundtrip_integer(unsigned int value);

int main(void) {
    // Test small value - should be lossless
    unsigned int small = 42;
    RoundtripResult r1 = roundtrip_integer(small);
    std::cout << "roundtrip(" << small << ") = " << r1.value
              << " | lossy: " << (int)r1.was_lossy << std::endl;

    // Test large value - may be lossy
    unsigned int large = 4294967295; // u32::MAX
    RoundtripResult r2 = roundtrip_integer(large);
    std::cout << "roundtrip(" << large << ") = " << r2.value
              << " | lossy: " << (int)r2.was_lossy << std::endl;

    return 0;
}
