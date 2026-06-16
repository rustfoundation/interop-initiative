
int add(int a, int b) {
    return a + b;
}

double add(double a, double b) {
    return a + b;
}

// extern "C" wrappers to expose the functions to Rust without name mangling
extern "C" {
    int add_int(int a, int b) {
        return add(a, b);
    }

    double add_double(double a, double b) {
        return add(a, b);
    }
}
