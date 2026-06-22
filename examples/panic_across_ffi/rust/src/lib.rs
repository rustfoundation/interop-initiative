// This example demonstrates why Rust panics must NOT cross FFI boundaries.
//
// We provide two functions:
//
// 1. trigger_panic (UNSAFE)
//    - Panics inside an extern "C" function
//    - Attempts to unwind across FFI boundary
//    - Rust aborts the program to prevent undefined behavior
//
// 2. trigger_panic_safe (SAFE)
//    - Uses catch_unwind to contain the panic
//    - Prevents unwinding into C++
//    - Program continues safely

// UNSAFE: Panic crosses FFI boundary

#[no_mangle]
pub extern "C" fn trigger_panic() {
    println!("Rust: about to panic (UNSAFE)...");

    // This panic occurs inside an extern "C" function.
    // Rust does NOT allow unwinding across FFI boundaries,
    // so it will abort the program.
    panic!("This panic crosses the FFI boundary!");
}

// SAFE: Panic is contained within Rust

#[no_mangle]
pub extern "C" fn trigger_panic_safe() {
    println!("Rust: about to panic (SAFE version)...");

    // catch_unwind prevents the panic from escaping Rust
    let result = std::panic::catch_unwind(|| {
        panic!("This panic is caught safely!");
    });

    // Handle the panic safely
    if result.is_err() {
        println!("Rust: panic was caught, no undefined behavior occurred.");
    }
}
