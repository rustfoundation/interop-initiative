//! A Rust library that exposes a Person struct to C++ via FFI
//!
//! Demonstrates passing structs across the FFI boundary using opaque pointers.
//! Person contains a nested Location struct — C++ never sees either struct
//! directly. It only holds a pointer and calls Rust functions to create,
//! inspect, and free the data.
//!
//! Also demonstrates the string return pattern: `get_person_info` returns a
//! Rust-allocated C string that must be freed with `release_get_person_info`.
//! C++ must never call `free()` or `delete` on it directly.
//!
//! ## Interop differences worth knowing
//!
//! - **Memory ownership:** Rust allocates with `Box`, which uses Rust's
//!   allocator. C++ must not call `free()` or `delete` on these pointers —
//!   only the matching Rust `release_*` function will use the right allocator.
//! - **Drop order:** when `release_person` drops a `Person`, Rust automatically
//!   drops `Location` too (fields drop in declaration order). C++ destructors
//!   work similarly, but the explicit `Drop` impl here lets us observe it.
//! - **String encoding:** `to_string_lossy` is used when reading C strings
//!   because C doesn't guarantee UTF-8. Rust `String` must be valid UTF-8, so
//!   invalid bytes become the replacement character (`\u{FFFD}`) rather than
//!   panicking or causing undefined behaviour.
//!
//! Note: all memory allocated by Rust must be freed by Rust. C++ must not
//! call `free()` or `delete` on pointers returned by these functions.
//!
//! Adapted from: <https://github.com/wisonye/rust-ffi-demo>

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uchar};

/// Gender enum — C++ passes an integer, Rust maps it to this
#[derive(Debug)]
pub enum Gender {
    Female,
    Male,
    Unknown,
}

/// A physical location — nested inside Person
///
/// C++ never sees this struct directly. It is owned by Person and freed
/// when the Person is released.
#[derive(Debug)]
pub struct Location {
    city: String,
    country: String,
}

/// A person with a name, gender, age, and location
pub struct Person {
    first_name: String,
    last_name: String,
    gender: Gender,
    age: u8,
    location: Location,
}

impl Person {
    fn print_info(&self) {
        println!(
            "Person {{ \"{} {}\", gender: {:?}, age: {}, city: \"{}\", country: \"{}\" }}",
            self.first_name,
            self.last_name,
            self.gender,
            self.age,
            self.location.city,
            self.location.country
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
/// `first_name`, `last_name`, `city`, and `country` must be valid, non-null,
/// null-terminated C strings.
/// The returned pointer must be freed by calling `release_person`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_person(
    first_name: *const c_char,
    last_name: *const c_char,
    gender: c_uchar,
    age: c_uchar,
    city: *const c_char,
    country: *const c_char,
) -> *mut Person {
    // to_string_lossy: C strings are not guaranteed to be valid UTF-8.
    // Rust Strings must be, so invalid bytes become the replacement character
    // rather than causing a panic or undefined behaviour.
    let first = unsafe { CStr::from_ptr(first_name) }
        .to_string_lossy()
        .into_owned();
    let last = unsafe { CStr::from_ptr(last_name) }
        .to_string_lossy()
        .into_owned();
    let city_str = unsafe { CStr::from_ptr(city) }
        .to_string_lossy()
        .into_owned();
    let country_str = unsafe { CStr::from_ptr(country) }
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
        location: Location {
            city: city_str,
            country: country_str,
        },
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
    // Box::from_raw pairs with Box::into_raw from create_person.
    // This ensures Rust's allocator frees the memory — calling C++'s
    // free() or delete here would be undefined behaviour.
    unsafe {
        drop(Box::from_raw(ptr));
    }
}

/// Returns a Rust-allocated C string describing the person
///
/// The string is allocated by Rust and must be freed by calling
/// `release_get_person_info`. C++ must not call `free()` or `delete` on it.
///
/// # Safety
///
/// The entire binary must only have one function with this un-mangled name.
/// Callers must use the C calling convention to call this function.
/// `ptr` must be a valid pointer returned by `create_person`, or null.
/// Returns null if `ptr` is null.
/// The returned pointer must be freed by calling `release_get_person_info`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_person_info(ptr: *const Person) -> *mut c_char {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    let person = unsafe { &*ptr };
    let info = format!(
        "{} {}, gender: {:?}, age: {}, city: {}, country: {}",
        person.first_name,
        person.last_name,
        person.gender,
        person.age,
        person.location.city,
        person.location.country
    );
    CString::new(info).unwrap_or_default().into_raw()
}

/// Frees a string returned by `get_person_info`
///
/// # Safety
///
/// The entire binary must only have one function with this un-mangled name.
/// Callers must use the C calling convention to call this function.
/// `ptr` must be a pointer returned by `get_person_info`, or null.
/// After calling this, the pointer must not be used again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn release_get_person_info(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(ptr));
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
        let city = CString::new("Lagos").unwrap();
        let country = CString::new("Nigeria").unwrap();

        let ptr = unsafe {
            create_person(
                first.as_ptr(),
                last.as_ptr(),
                0,
                30,
                city.as_ptr(),
                country.as_ptr(),
            )
        };
        assert!(!ptr.is_null());

        unsafe { print_person(ptr) };
        unsafe { release_person(ptr) };
    }

    #[test]
    fn test_get_and_release_info() {
        let first = CString::new("Bob").unwrap();
        let last = CString::new("Jones").unwrap();
        let city = CString::new("Abuja").unwrap();
        let country = CString::new("Nigeria").unwrap();

        let ptr = unsafe {
            create_person(
                first.as_ptr(),
                last.as_ptr(),
                1,
                25,
                city.as_ptr(),
                country.as_ptr(),
            )
        };
        assert!(!ptr.is_null());

        let info_ptr = unsafe { get_person_info(ptr) };
        assert!(!info_ptr.is_null());

        // Read the string back to verify it contains the expected data
        let info = unsafe { CStr::from_ptr(info_ptr) }.to_string_lossy();
        assert!(info.contains("Bob Jones"));
        assert!(info.contains("Abuja"));

        unsafe { release_get_person_info(info_ptr) };
        unsafe { release_person(ptr) };
    }

    #[test]
    fn test_null_safety() {
        // These should not crash
        unsafe { print_person(std::ptr::null()) };
        unsafe { release_person(std::ptr::null_mut()) };
        let null_info = unsafe { get_person_info(std::ptr::null()) };
        assert!(null_info.is_null());
        unsafe { release_get_person_info(std::ptr::null_mut()) };
    }

    #[test]
    fn test_gender_variants() {
        // Verify all three gender values map correctly and produce a valid pointer
        let first = CString::new("Test").unwrap();
        let last = CString::new("User").unwrap();
        let city = CString::new("London").unwrap();
        let country = CString::new("UK").unwrap();

        for gender_code in [0u8, 1u8, 2u8] {
            let ptr = unsafe {
                create_person(
                    first.as_ptr(),
                    last.as_ptr(),
                    gender_code,
                    20,
                    city.as_ptr(),
                    country.as_ptr(),
                )
            };
            assert!(!ptr.is_null(), "gender code {gender_code} returned null");
            unsafe { release_person(ptr) };
        }
    }

    #[test]
    fn test_info_contains_location() {
        // get_person_info output must include both city and country
        let first = CString::new("Zara").unwrap();
        let last = CString::new("Ahmed").unwrap();
        let city = CString::new("Cairo").unwrap();
        let country = CString::new("Egypt").unwrap();

        let ptr = unsafe {
            create_person(
                first.as_ptr(),
                last.as_ptr(),
                0,
                22,
                city.as_ptr(),
                country.as_ptr(),
            )
        };
        assert!(!ptr.is_null());

        let info_ptr = unsafe { get_person_info(ptr) };
        assert!(!info_ptr.is_null());

        let info = unsafe { CStr::from_ptr(info_ptr) }.to_string_lossy();
        assert!(info.contains("Cairo"), "info missing city: {info}");
        assert!(info.contains("Egypt"), "info missing country: {info}");

        unsafe { release_get_person_info(info_ptr) };
        unsafe { release_person(ptr) };
    }
}
