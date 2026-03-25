use overloaded_cpp_c_abi_wrappers::{call_overload_f64, call_overload_i32};

fn main() {
    // Example inputs for each overload path.
    let i32_input = 21;
    let f64_input = 21.0;

    // Call into C++ through Rust's safe wrapper functions.
    let i32_result = call_overload_i32(i32_input);
    let f64_result = call_overload_f64(f64_input);

    // Show which overload we intended to call and the returned value.
    println!("double_value({i32_input}) [int overload] -> {i32_result}");
    println!("double_value({f64_input:.1}) [double overload] -> {f64_result:.1}");
}
