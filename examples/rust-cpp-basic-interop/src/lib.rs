use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
pub struct LogMessage {
    pub msg: *const c_char,
    pub time: i64,
    pub check: bool,
}

#[no_mangle]
pub extern "C" fn log_message(msg: *const c_char) {
    unsafe { print_log("[INFO]", msg); }
}

#[no_mangle]
pub extern "C" fn log_warning(msg: *const c_char) {
    unsafe { print_log("[WARNING]", msg); }
}

#[no_mangle]
pub extern "C" fn log_error(msg: *const c_char) {
    unsafe { print_log("[ERROR]", msg); }
}

#[no_mangle]
pub unsafe extern "C" fn log_struct(log: *const LogMessage) {
    if log.is_null() {
        return;
    }
    let log_ref = &*log;
    let c_str = CStr::from_ptr(log_ref.msg);
    let message = c_str.to_str().unwrap();

    println!("Message: {}", message);
    println!("Time: {}", log_ref.time);
    println!("Check: {}", log_ref.check);
}

// Internal helper function (not exposed to C++)
unsafe fn print_log(level: &str, msg: *const c_char) {
    let c_str = CStr::from_ptr(msg);
    let str_slice = c_str.to_str().unwrap();

    println!("{} {}", level, str_slice);
}
