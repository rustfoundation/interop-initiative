// this runs before the actual build — it compiles our c++ code
// so that rust can link against it later
fn main() {
    cc::Build::new()
        .cpp(true)           // we're compiling c++ not plain c
        .file("main.cpp")   // our c++ file with the wrapper functions
        .compile("greetings"); // output lib name, rust will find it automatically
}