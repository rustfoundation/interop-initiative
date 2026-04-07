use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
/// # Safety
/// - a and b are non null pointers (*const c_char).
pub unsafe extern "C" fn swap_pointers(a: *mut *const c_char, b: *mut *const c_char) {
    std::ptr::swap(a, b);
}

#[no_mangle]
/// # Safety
/// - a and b are writable non-null pointers to buffers.
/// - a_size and b_size must be less than or equal to the allocated sizes of a and b.
/// - Buffers a and b must not overlap.
pub unsafe extern "C" fn swap_strings(a: *mut c_char, b: *mut c_char, a_size: u32, b_size: u32) {
    let a_slice = slice::from_raw_parts_mut(a as *mut u8, a_size as usize);
    let b_slice = slice::from_raw_parts_mut(b as *mut u8, b_size as usize);

    let a_str = CStr::from_ptr(a).to_str().unwrap().to_string();
    let b_str = CStr::from_ptr(b).to_str().unwrap().to_string();

    assert!(b_str.len() < a_size as usize, "a buffer too small");
    assert!(a_str.len() < b_size as usize, "b buffer too small");

    // copy b into a
    b_str
        .as_bytes()
        .iter()
        .enumerate()
        .for_each(|(i, &byte)| a_slice[i] = byte);
    a_slice[b_str.len()] = 0;
    a_slice
        .iter_mut()
        .skip(b_str.len() + 1)
        .for_each(|byte| *byte = 0);

    // copy a into b
    a_str
        .as_bytes()
        .iter()
        .enumerate()
        .for_each(|(i, &byte)| b_slice[i] = byte);
    b_slice[a_str.len()] = 0;
    b_slice
        .iter_mut()
        .skip(a_str.len() + 1)
        .for_each(|byte| *byte = 0);
}
