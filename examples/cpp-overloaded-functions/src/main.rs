// These match the extern C wrapper functions in overloads.cpp.
// this will give Rust the function signatures so it knows how to call them.
// we use c_int and c_double instead of i32 and f64 because the C standard
// doesn't guarantee the exact size of int or double on every platform.
use std::ffi::{c_double, c_int};

extern "C" {
    fn add_int(a: c_int, b: c_int) -> c_int;
    fn add_double(a: c_double, b: c_double) -> c_double;
}

fn main() {
    // FFI calls are unsafe because Rust has no way to check that the
    // C++ side is doing the right thing with memory or types.
    let int_result = unsafe { add_int(3, 4) };
    let double_result = unsafe { add_double(1.5, 2.5) };

    println!("add_int(3, 4) = {}", int_result);
    println!("add_double(1.5, 2.5) = {}", double_result);
}
