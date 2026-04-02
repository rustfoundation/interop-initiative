fn main() {
    cc::Build::new()
        .file("../cpp/multiply.cpp")
        .cpp(true)
        .compile("multiply");
}
