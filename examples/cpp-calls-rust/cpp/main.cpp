#include <iostream>

// Declare the Rust function using C ABI
extern "C" int add(int left, int right);


// Expose C++ function to Rust
extern "C" int multiply(int a, int b) {
	return a * b;
}

int main() {
	std::cout << "2 + 3 = " << add(2, 3) << std::endl;
	return 0;
}
