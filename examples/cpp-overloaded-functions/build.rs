// This runs before the Rust code compiles. It uses the cc crate to
// build overloads.cpp into a static library that gets linked in automatically.
fn main() {
    cc::Build::new()
        .cpp(true) // i used a C++ compiler instead of a C compiler because the .cpp(true) tells cc crate to invoke g++/clang++ instead of gcc/clang.
        .file("overloads.cpp")
        .compile("overloads");
}
