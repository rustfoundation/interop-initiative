// This program calls overloaded C++ functions from Rust.
// Since Rust doesn't support overloading, we use extern "C"
// wrapper functions with unique names as a workaround.

use std::ffi::{c_double, c_int};

// Declare the C++ wrapper functions so Rust knows they exist.
// These must match the extern "C" functions in add.cpp exactly.
unsafe extern "C" {
    // Calls the C++ add(int, int) overload
    fn add_int(a: c_int, b: c_int) -> c_int;
    // Calls the C++ add(double, double) overload
    fn add_double(a: c_double, b: c_double) -> c_double;

    fn add_int3(a: c_int, b: c_int, c: c_int) -> c_int;
}

fn main() {
    // Call the integer overload
    let int_result = unsafe { add_int(12, 33) };
    println!("add_int(12, 33) = {int_result}");

    // Call the double overload
    let double_result = unsafe { add_double(2.5, 5.7) };
    println!("add_double(2.5, 5.7) = {double_result}");

    // Overloading on number of arguments:
    // Same function name "add" in C++, but takes three arguments instead of two
    let three_arg_result = unsafe { add_int3(10, 20, 30) };
    println!("add_int3(10, 20, 30) = {three_arg_result}");
}
