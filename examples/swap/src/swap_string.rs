use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn swap_string(a: *mut *mut c_char, b: *mut *mut c_char) {
    unsafe {
	let temp = *a;
	*a = *b;
	*b = temp;
	}
}
