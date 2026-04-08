// A C++ program that creates and uses a Rust Person struct via FFI
//
// Person contains a nested Location struct (city and country). C++ never
// sees either struct — it passes the city and country as plain strings when
// creating the person, and Rust builds the nested Location internally.
//
// Also demonstrates the string return pattern: get_person_info returns a
// Rust-allocated string. C++ reads it, then calls release_get_person_info
// to free it. C++ must never call free() or delete on it directly.
//
// Adapted from: https://github.com/wisonye/rust-ffi-demo

#include <cstdint>
#include <iostream>

// Opaque type — C++ never sees inside this struct
struct Person;

// Declare the Rust FFI functions
extern "C" {
    Person *create_person(const char *first_name, const char *last_name,
                          uint8_t gender, uint8_t age,
                          const char *city, const char *country);
    void print_person(const Person *ptr);
    char *get_person_info(const Person *ptr);
    void release_get_person_info(char *ptr);
    void release_person(Person *ptr);
}

int main(void) {
    std::cout << "Creating a Person in Rust from C++..." << std::endl;

    // gender: 0 = Female, 1 = Male, 2+ = Unknown
    // city and country are plain strings — Rust builds the Location internally
    Person *alice = create_person("Alice", "Smith", 0, 30, "Lagos", "Nigeria");

    std::cout << "Printing person info (via Rust):" << std::endl;
    print_person(alice);

    // get_person_info returns a Rust-allocated string.
    // C++ reads it, then hands it back to Rust to free.
    std::cout << "Getting person info as a string (Rust allocates):" << std::endl;
    char *info = get_person_info(alice);
    std::cout << info << std::endl;
    std::cout << "Freeing the info string (Rust frees):" << std::endl;
    release_get_person_info(info);

    std::cout << "Freeing the person (Rust Drop runs):" << std::endl;
    release_person(alice);

    return 0;
}
