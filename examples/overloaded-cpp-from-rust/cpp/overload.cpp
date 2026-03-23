#include <cstdint>

// First overload: doubles an integer.
int double_value(int value) {
    return value * 2;
}

// Second overload: doubles a floating-point value.
double double_value(double value) {
    return value * 2.0;
}

// C ABI wrapper with a unique name that Rust can call.
extern "C" int double_value_i32(int value) {
    return double_value(value);
}

// C ABI wrapper with a unique name that Rust can call.
extern "C" double double_value_f64(double value) {
    return double_value(value);
}
