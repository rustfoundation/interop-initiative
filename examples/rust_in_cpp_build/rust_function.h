// included guard to  prevents this file from being loaded twice
#ifndef RUST_FUNCTIONS_H
#define RUST_FUNCTIONS_H

extern "C" {
    // must match the extern "C" on the Rust side so both agree on calling conventions
    int add(int a, int b); //here we have  declaration only, the  actual code lives in the Rust lib
}

#endif