all:
	cargo build --release
	clang++ main.cpp target/release/librust_interop.a -o main

run:
	./main

clean:
	rm -f main	
