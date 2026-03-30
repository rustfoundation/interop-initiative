use overloaded_cpp_c_abi_wrappers::{call_overload_double, call_overload_int};
use std::ffi::{c_double, c_int};

fn main() {
    // Example inputs for each overload path.
    let int_input: c_int = 21;
    let double_input: c_double = 21.0;

    // Call into C++ through Rust's safe wrapper functions.
    let int_result = call_overload_int(int_input);
    let double_result = call_overload_double(double_input);

    // Show which overload we intended to call and the returned value.
    println!("multiply_2({int_input}) [int overload] -> {int_result}");
    println!("multiply_2({double_input:.1}) [double overload] -> {double_result:.1}");
}
