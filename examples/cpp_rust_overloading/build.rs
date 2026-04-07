fn main() {
    // Compile src/lib.cpp as C++ and link it
    cc::Build::new()
        .cpp(true) // Crucial: treat file as C++
        .file("src/lib.cpp")
        .compile("overloads");
}
