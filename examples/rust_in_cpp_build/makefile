all: main # this is the default target, it builds main

# if there are any changes , it rebuilds these three files change
main: main.cpp lib.rs rust_function.h
   # Compile Rust code into a static library that C++ can link against
	rustc --crate-type=staticlib lib.rs -o librust_functions.a 
    # Compile C++ and link it with the Rust library to create the final program
	c++ main.cpp -L. -lrust_functions -o main
# Here I Remove compiled files to start afresh
clean:
	rm -f main librust_functions.a