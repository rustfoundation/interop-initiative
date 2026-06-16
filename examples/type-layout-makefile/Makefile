# Makefile: Build a C program that calls into a Rust library.
#
# This example demonstrates how to integrate a Rust static library into
# an existing C build system using only make, cargo, and gcc.
#
# Usage:
#   make        - build and run the example
#   make clean  - remove all build artifacts
CC ?= gcc 

RUST_DIR   := .
C_SRC      := main.c
TARGET     := main
LIB_DIR    := target/release
LIB_NAME   := libtype_layout_example.a

# Detect the correct linker flags for the Rust standard library
RUST_LIBS  := $(shell rustc --print=native-static-libs 2>&1 | grep "note:" | sed 's/note: native-static-libs: //')

.PHONY: all clean

all: $(TARGET)
	@echo "--- Running example ---"
	./$(TARGET)

# Step 1: build the Rust static library
$(LIB_DIR)/$(LIB_NAME):
	cargo build --release --manifest-path=$(RUST_DIR)/Cargo.toml

# Step 2: compile main.c and link against the Rust static library
$(TARGET): $(C_SRC) $(LIB_DIR)/$(LIB_NAME)
	$(CC) $(C_SRC) \
		-L$(LIB_DIR) \
		-ltype_layout_example \
		$(RUST_LIBS) \
		-o $(TARGET)

clean:
	cargo clean --manifest-path=$(RUST_DIR)/Cargo.toml
	rm -f $(TARGET)
