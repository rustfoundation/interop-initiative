// In C++, we can have two functions with the same name as long as
// they take different types. This is called overloading.
int add(int a, int b) {
    return a + b;
}

double add(double a, double b) {
    return a + b;
}

// Rust can't call C++ overloads directly it only understands C style functions,
// and in C you can't have two functions with the same name.
// So we create wrapper functions with different names, and use extern "C"
// to make sure the compiler doesn't rename them behind the scenes.
extern "C" {
    int add_int(int a, int b) {
        return add(a, b);
    }

    double add_double(double a, double b) {
        return add(a, b);
    }
}
