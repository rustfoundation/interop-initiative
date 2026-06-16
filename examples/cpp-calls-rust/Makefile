# Build Rust and C++ parts

all:
	cd rust && cargo build --release

	g++ cpp/main.cpp cpp/multiply.cpp -Wl,--start-group -Lrust/target/release -lrustlib -o main -Wl,--end-group


run: all
	./main

clean: 
	rm -f main
	
	cd rust && cargo clean
