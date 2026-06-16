// Two overloaded C++ functions with the same name "add"
// C++ knows which one to call based on the argument types

// This overload takes two integers and returns an integer
int add(int a, int b) {
    return a + b;
}

// This overload takes two doubles (decimal numbers) and returns a double
double add(double a, double b) {
    return a + b;
}

// This overload takes three integers — overloading on number of arguments
int add(int a, int b, int c) {
    return a + b + c;
}

// Rust can't call the overloaded functions directly because:
// 1. C++ mangles the names (adds type info to the symbol name)
// 2. Rust doesn't support having two functions with the same name
//
// Workaround: extern "C" wrappers with unique names
// extern "C" stops C++ from mangling the name, so Rust can find them

extern "C" int add_int(int a, int b) {
    return add(a, b);    // C++ picks the int overload because a and b are ints
}

extern "C" double add_double(double a, double b) {
    return add(a, b);    // C++ picks the double overload because a and b are doubles
}

//this is the  Wrapper for the three-argument overload
extern "C" int add_int3(int a, int b, int c) {
    return add(a, b, c);
}
