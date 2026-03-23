// These match the extern C wrapper functions in overloads.cpp.
// this will give Rust the function signatures so it knows how to call them.
extern "C" {
    fn add_int(a: i32, b: i32) -> i32;
    fn add_double(a: f64, b: f64) -> f64;
}

fn main() {
    // FFI calls are unsafe because Rust has no way to check that the
    // C++ side is doing the right thing with memory or types.
    let int_result = unsafe { add_int(3, 4) };
    let double_result = unsafe { add_double(1.5, 2.5) };

    println!("add_int(3, 4) = {}", int_result);
    println!("add_double(1.5, 2.5) = {}", double_result);
}
