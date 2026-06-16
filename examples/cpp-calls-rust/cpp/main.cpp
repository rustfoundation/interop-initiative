#include <iostream>

// Declare the Rust function using C ABI
extern "C" int add(int left, int right);

int main() {
	std::cout << "2 + 3 = " << add(2, 3) << std::endl;
	return 0;
}
