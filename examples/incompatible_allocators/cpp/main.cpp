#include <cstddef>
#include <cstdint>
#include <iostream>

// Function provided by Rust
extern "C" void pass_to_cpp();

// This function is called by Rust and receives ownership of memory.
//
// BUG DEMONSTRATION:
// The memory was allocated in Rust using its allocator,
// but we incorrectly deallocate it using C++'s operator delete.
//
// This mismatch leads to undefined behavior.
extern "C" void take_ownership_in_cplusplus(uint32_t* pointer, size_t length) {
    std::cout << "C++ received array of length: " << length << std::endl;

    // (Optional) Access data safely (just to show it's valid initially)
    std::cout << "First element: " << pointer[0] << std::endl;

    //  WRONG: Deallocating with C++ allocator
    //
    // Why this is UB:
    // - Rust allocator != C++ allocator
    // - Metadata layout may differ
    // - Size assumptions may differ
    //
    // This can cause:
    // - Heap corruption
    // - Crashes
    // - Silent memory bugs
    ::operator delete(pointer, length * sizeof(*pointer));
}

int main() {
    std::cout << "Calling Rust code..." << std::endl;

    // This will trigger the whole flow:
    // Rust allocates → passes pointer → C++ frees incorrectly
    pass_to_cpp();

    std::cout << "Finished execution." << std::endl;
    return 0;
}