all: build run

build:
	cargo build --release
	gcc main.c target/release/liballocator_interop_example.a -o main

run:
	./main