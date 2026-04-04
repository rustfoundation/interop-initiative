use cpp::cpp;
use std::ffi::{c_double, c_int};

/*
  This specifies the Application Binary Interface (ABI).
  It tells Rust to use the standard C calling convention for passing arguments,
  representing data and receiving return values.
  It is marked as unsafe because the responsiblity is on us to ensure the functions
  signatures exactly match the actual C code
*/
unsafe extern "C" {
    fn multiply_2_int(value: c_int) -> c_int;
    fn multiply_2_double(value: c_double) -> c_double;
}

// Here goes our cpp code where we implement our overloaded set functions
// The functins differ in ther arg and return types
cpp! {{
    // First overload: doubles an integer.
    int multiply_2(int value) {
        return value * 2;
    }

    // Second overload: doubles a floating-point value.
    double multiply_2(double value) {
        return value * 2.0;
    }

    /*
      Since C++ doesn't have a stable ABI and
      rust would also need different resolved names to use the overloaded set,
      we wrap the c++ overloaded set in C ABI wrappers where we do manual name resolution.
    */
    extern "C" {
        // Unique name for when int is passed as argument.
        int multiply_2_int(int value) {
            return multiply_2(value);
        }

        // Unique name for when double is passed as argument.
        double multiply_2_double(double value) {
            return multiply_2(value);
        }
    }
}}

/// Calls the wrapper that forwards to the C++ `int` overload.
fn call_overload_int(value: c_int) -> c_int {
    // SAFETY: The symbol is defined above in the embedded C++ block.
    unsafe { multiply_2_int(value) }
}

/// Calls the wrapper that forwards to the C++ `double` overload.
fn call_overload_double(value: c_double) -> c_double {
    // SAFETY: The symbol is defined above in the embedded C++ block.
    unsafe { multiply_2_double(value) }
}

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

#[cfg(test)]
mod tests {
    use super::{call_overload_double, call_overload_int};

    #[test]
    fn calls_int_overload() {
        assert_eq!(call_overload_int(21), 42);
    }

    #[test]
    fn calls_double_overload() {
        let result = call_overload_double(21.0);
        assert!((result - 42.0).abs() < f64::EPSILON);
    }
}
