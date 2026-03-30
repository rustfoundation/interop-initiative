// First overload: doubles an integer.
int multiply_2(int value) {
    return value * 2;
}

// Second overload: doubles a floating-point value.
double multiply_2(double value) {
    return value * 2.0;
}

// C ABI wrappers with unique names that Rust can call.
extern "C" {
    // Unique name for when int is passed as argument
    int multiply_2_int(int value) {
        return multiply_2(value);
    }

    // Unique name for when double is passed as argument
    double multiply_2_double(double value) {
        return multiply_2(value);
    }
}