# Build Rust and C++ parts

all:
	cd rust && cargo build --release

	g++ cpp/main.cpp -Lrust/target/release -lrustlib -o main


run: all
	./main

clean: 
	rm -f main
	
	cd rust && cargo clean
