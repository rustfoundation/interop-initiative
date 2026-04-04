//! A Rust library that exposes a Person struct to C++ via FFI
//!
//! Demonstrates passing structs across the FFI boundary using opaque pointers.
//! C++ never sees inside the Rust struct — it only holds a pointer and calls
//! Rust functions to create, inspect, and free the data.
//!
//! Note: memory allocated by Rust must be freed by Rust. C++ must not call
//! `free()` or `delete` on pointers returned by these functions.
//!
//! Adapted from: <https://github.com/wisonye/rust-ffi-demo>

use std::ffi::CStr;
use std::os::raw::{c_char, c_uchar};

/// Gender enum — C++ passes an integer, Rust maps it to this
#[derive(Debug)]
pub enum Gender {
    Female,
    Male,
    Unknown,
}

/// A person with a name, gender, and age
pub struct Person {
    first_name: String,
    last_name: String,
    gender: Gender,
    age: u8,
}

impl Person {
    fn print_info(&self) {
        println!(
            "Person {{ first_name: \"{}\", last_name: \"{}\", gender: {:?}, age: {} }}",
            self.first_name, self.last_name, self.gender, self.age
        );
    }
}

/// Prints a message when a Person is freed, so we can verify cleanup works
impl Drop for Person {
    fn drop(&mut self) {
        println!("Person dropped: {} {}", self.first_name, self.last_name);
    }
}

// ---------------------------------------------------------------------------
// FFI functions
// ---------------------------------------------------------------------------

/// Creates a new Person on the heap and returns an opaque pointer
///
/// # Safety
///
/// The entire binary must only have one function with this un-mangled name.
/// Callers must use the C calling convention to call this function.
/// `first_name` and `last_name` must be valid, non-null, null-terminated C strings.
/// The returned pointer must be freed by calling `release_person`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_person(
    first_name: *const c_char,
    last_name: *const c_char,
    gender: c_uchar,
    age: c_uchar,
) -> *mut Person {
    let first = unsafe { CStr::from_ptr(first_name) }
        .to_string_lossy()
        .into_owned();
    let last = unsafe { CStr::from_ptr(last_name) }
        .to_string_lossy()
        .into_owned();

    let g = match gender {
        0 => Gender::Female,
        1 => Gender::Male,
        _ => Gender::Unknown,
    };

    let person = Person {
        first_name: first,
        last_name: last,
        gender: g,
        age,
    };

    Box::into_raw(Box::new(person))
}

/// Prints the person's info to stdout
///
/// # Safety
///
/// The entire binary must only have one function with this un-mangled name.
/// Callers must use the C calling convention to call this function.
/// `ptr` must be a valid pointer returned by `create_person`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_person(ptr: *const Person) {
    if ptr.is_null() {
        return;
    }
    unsafe { &*ptr }.print_info();
}

/// Frees a Person previously created by `create_person`
///
/// # Safety
///
/// The entire binary must only have one function with this un-mangled name.
/// Callers must use the C calling convention to call this function.
/// `ptr` must be a valid pointer returned by `create_person`, or null.
/// After calling this, the pointer must not be used again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn release_person(ptr: *mut Person) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_create_and_release() {
        let first = CString::new("Alice").unwrap();
        let last = CString::new("Smith").unwrap();

        let ptr = unsafe { create_person(first.as_ptr(), last.as_ptr(), 0, 30) };
        assert!(!ptr.is_null());

        unsafe { print_person(ptr) };
        unsafe { release_person(ptr) };
    }

    #[test]
    fn test_null_safety() {
        // These should not crash
        unsafe { print_person(std::ptr::null()) };
        unsafe { release_person(std::ptr::null_mut()) };
    }
}
