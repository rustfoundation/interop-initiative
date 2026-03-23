all:
	cargo build
run:
	@clang swap_string.c target/debug/libswap_string.a -o swap_string
	@./swap_string
clean:
	cargo clean
	rm -f swap_string
