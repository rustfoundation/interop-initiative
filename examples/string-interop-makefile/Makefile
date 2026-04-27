# Makefile: Pass strings between C and a Rust library.
#
# This example demonstrates a common FFI pitfall: strings allocated by Rust
# must be freed by Rust, not by C's free(). Using the wrong deallocator
# causes undefined behaviour (see problem-space/0001-incompatible-allocators.md).
#
# Usage:
#   make        - build and run the example
#   make clean  - remove all build artifacts

RUST_DIR  := .
C_SRC     := main.c
TARGET    := main
LIB_DIR   := target/release
LIB_NAME  := libstring_interop_example.a

RUST_LIBS := $(shell rustc --print=native-static-libs 2>&1 | grep "note:" | sed 's/note: native-static-libs: //')

.PHONY: all clean

all: $(TARGET)
	@echo "--- Running example ---"
	./$(TARGET)

$(LIB_DIR)/$(LIB_NAME):
	cargo build --release --manifest-path=$(RUST_DIR)/Cargo.toml

$(TARGET): $(C_SRC) $(LIB_DIR)/$(LIB_NAME)
	gcc $(C_SRC) \
		-L$(LIB_DIR) \
		-lstring_interop_example \
		$(RUST_LIBS) \
		-o $(TARGET)

clean:
	cargo clean --manifest-path=$(RUST_DIR)/Cargo.toml
	rm -f $(TARGET)
