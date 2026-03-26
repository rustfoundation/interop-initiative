use std::os::raw::c_char;

#[no_mangle]
/// # Safety
///
/// a and b must be valid, non null pointers.
/// They must point to valid memory containing *must_char.
/// The memory must be properly aligned and writable.
pub unsafe extern "C" fn swap_string(a: *mut *mut c_char, b: *mut *mut c_char) {
    core::ptr::swap(a, b);
}
