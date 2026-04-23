CC ?= clang++

all:
	cargo build --release
	$(CC) main.cpp target/release/librust_interop.a -o main

run:
	./main

clean:
	rm -f main	
