# Build the Rust library first, then compile C++ and link them together

all: fibonacci_demo

# Step 1: Build Rust as a static library
libfibonacci.a:
	rustc --crate-type=staticlib -o libfibonacci.a lib.rs

# Step 2: Compile C++ and link with Rust library
fibonacci_demo: libfibonacci.a
	g++ -o fibonacci_demo main.cpp -L. -lfibonacci -lpthread -ldl

# Clean build files
clean:
	rm -f fibonacci_demo libfibonacci.a

run: fibonacci_demo
	./fibonacci_demo
