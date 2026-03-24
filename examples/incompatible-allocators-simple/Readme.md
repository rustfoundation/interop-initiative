# Incompatible Allocators Example

This example shows undefined behavior when memory allocated in Rust is deallocated in C++ using a different allocator.

## What happens

- Rust allocates memory using its global allocator
- Ownership is transferred to C++
- C++ incorrectly frees the memory using a C allocator `free()`

## Why this is wrong
Rust and C++ may use different memory allocators.
If memory allocated in Rust is freed in C++ (or vice versa), it can lead to:
- Undefined behavior.
- Memory corruption.
- Crashes (in real-world scenarios).

### Compile C++
g++ -c cpp.cpp -o cpp.o

### Compile Rust and Link with C++
rustc main.rs -C link-arg=cpp.o -l c++

### Run the Program
./main

## Note
This may appear working but it is undefined behaviour and unsafe.