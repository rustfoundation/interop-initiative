# Build the Rust library using cargo, then compile and link C++

all: fibonacci_demo

# Step 1: Build Rust library using cargo
rust-library/target/debug/libfibonacci.a:
	cd rust-library && cargo build

# Step 2: Compile C++ and link with Rust library
fibonacci_demo: rust-library/target/debug/libfibonacci.a
	g++ -o fibonacci_demo main.cpp rust-library/target/debug/libfibonacci.a -lpthread -ldl

# Clean build files
clean:
	rm -f fibonacci_demo
	cd rust-library && cargo clean

run: all
	./fibonacci_demo
