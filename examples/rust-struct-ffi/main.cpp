// A C++ program that creates and uses a Rust Person struct via FFI
//
// The Person struct is opaque — C++ only holds a pointer to it and uses
// Rust functions to create, inspect, and free it. This is the standard
// pattern for passing heap-allocated Rust data to C++.
//
// Adapted from: https://github.com/wisonye/rust-ffi-demo

#include <cstdint>
#include <iostream>

// Opaque type — C++ never sees inside this struct
struct Person;

// Declare the Rust FFI functions
extern "C" {
    Person *create_person(const char *first_name, const char *last_name,
                          uint8_t gender, uint8_t age);
    void print_person(const Person *ptr);
    void release_person(Person *ptr);
}

int main(void) {
    std::cout << "Creating a Person in Rust from C++..." << std::endl;

    // gender: 0 = Female, 1 = Male, 2+ = Unknown
    Person *alice = create_person("Alice", "Smith", 0, 30);

    std::cout << "Printing person info (via Rust):" << std::endl;
    print_person(alice);

    std::cout << "Freeing the person (Rust Drop runs):" << std::endl;
    release_person(alice);

    return 0;
}
