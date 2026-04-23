use std::ffi::{c_double, c_int};

extern "C" {
    fn add_int(a: c_int, b: c_int) -> c_int;
    fn add_double(a: c_double, b: c_double) -> c_double;
}

fn main() {
    println!("Testing C++ Overloads from Rust");

    unsafe {
        // Call add(int, int) in C++
        let sum_int = add_int(10, 20);
        println!("  add_int(10, 20)       = {}", sum_int);

        // Call add(double, double) in C++
        let sum_double = add_double(1.5, 2.75);
        println!("  add_double(1.5, 2.75) = {}", sum_double);
    }
}
