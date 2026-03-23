
// telling rust about the c wrapper functions we made in main.cpp
// since rust doesn't know about c++ overloading, we use the unique names
extern "C"{
    fn greet_number(x: i32);
    fn greet_name(x: *const std::ffi::c_char);
}

fn main(){
    // gotta use unsafe here bc rust can't check ffi stuff
    unsafe { greet_number(4)};
    // c"..." gives us a c string literal, .as_ptr() turns it into a raw pointer
    unsafe {greet_name(c"Yash".as_ptr());}
}
