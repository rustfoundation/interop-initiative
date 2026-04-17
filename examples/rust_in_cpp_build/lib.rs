// no_mangle so C++ can actually find this function by name after linking
#[unsafe(no_mangle)] 
//using pub here  so the function can be  visible outside the file and  C++ can see it 
// `extern "C"` makes Rust use the C calling convention, So that  C++ understands.
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b // Implicit return here
}