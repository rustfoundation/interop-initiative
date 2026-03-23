use std::ffi::CStr;
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn log_message(msg: *const c_char) {
    print_log("[INFO]", msg);
}

#[unsafe(no_mangle)]
pub extern "C" fn log_warning(msg: *const c_char) {
    print_log("[WARNING]", msg);
}

#[unsafe(no_mangle)]
pub extern "C" fn log_error(msg: *const c_char) {
    print_log("[ERROR]", msg);
}

// Internal helper function (not exposed to C++)
fn print_log(level: &str, msg: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(msg) };
    let str_slice = c_str.to_str().unwrap();

    println!("{} {}", level, str_slice);
}
