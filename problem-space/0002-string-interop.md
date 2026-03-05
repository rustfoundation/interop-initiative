- Problem Name: string_interop
- Start Date: 2026-02-18
- Problem Statement PR: [rustfoundation/interop-initiative#1](https://github.com/rustfoundation/interop-initiative/pull/1)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)
- C++ Issue: [????/????#0000](https://github.com/????/????/pull/0000)

## Summary
[summary]: #summary

Rust and C++ have different owned string and string slice types, `String`/`string` and `str`/`string_view`.

These types have different memory layouts, valid character constraints, operations, and operational semantics.
In both languages, type layouts are implementation-defined, and can vary based on the compiler, platform, and the origin of the string data (C++).

### C++ to Rust

Most Rust library APIs use the `String` or `&str` types, rather than traits abstracting string storage and operations.
Converting an arbitrary-length string for every C++ to Rust call is expensive. And the conversion may need to handle invalid characters.
Using `CString` or `OsString` avoids character validity issues, but significantly reduces API compatibility, because fewer APIs support those types.

### Rust to C++

On the C++ side, the situation is similar, many APIs take `string` or `string_view`, with fixed `char_traits`.
But other APIs take the `basic_string` or `basic_string_view` templates, which could technically wrap Rust UTF-8 strings.
However, maintaining consistent semantics across all string operations is challenging, particularly for writeable strings.

### Example Code
[example-code]: #example-code

This example is similar to the one in [the unique ownership problem statement](0004-unique-ownership.md), but it uses pointers to avoid undefined behaviour.

The Rust code passes some Rust `CStr` pointers to C++.
The C++ code constructs larger `std::string`s by copying their data bytes from the Rust pointer into a new allocation, which is an unacceptable performance trade-off for some use cases.

```rust
#![recursion_limit = "256"]

use cpp::cpp;
use std::ffi::c_char;

fn main() {
    unsafe {
        pass_strings_to_cplusplus(c"a".as_ptr(), c"not small string optimized".as_ptr());
    }
}

unsafe extern "C" {
    fn pass_strings_to_cplusplus(cstr: *const c_char, cother: *const c_char);
}

cpp! {{
    #include <string>
    #include <cstring>
    #include <iostream>

    extern "C" {
        extern const size_t SIZE_OF_CPP_STRING;

        void use_strings_in_rust(const std::basic_string<char> *cppstr, const std::basic_string<char> *other);
    }

    extern "C" void pass_strings_to_cplusplus(const char *cstr, const char *cother) {
        auto cppstr = std::string(cstr, strlen(cstr));
        auto other = std::string(cother, strlen(cother));

        // Required for FFI safety by Rust (fails at compile time)
        //static_assert(std::is_trivially_move_constructible_v<typeof(cppstr)>);
        if (SIZE_OF_CPP_STRING != sizeof(cppstr)) { std::cout << "Size is " << SIZE_OF_CPP_STRING << " in Rust but " << sizeof(cppstr) << " in C++" << std::endl; abort(); }

        use_strings_in_rust(&cppstr, &other);
    }
}}

#[unsafe(no_mangle)]
static SIZE_OF_CPP_STRING: usize = 24;

#[repr(C)]
struct CppString {
    data: [u8; SIZE_OF_CPP_STRING],
}

#[unsafe(no_mangle)]
unsafe fn use_strings_in_rust(cppstr: *const CppString, other: *const CppString) {
    println!("{cppstr:?}, {other:?}")
}
```

This code can be run using the [`cpp` crate](https://docs.rs/cpp/latest/cpp/macro.cpp.html), like the [Build and Run code example](https://github.com/rustfoundation/interop-initiative/issues/5).

## Related Problems
[related-problems]: #related-problems

Creating and modifying strings at runtime depends on:

- [correct memory allocation and deallocation](0001-incompatible-allocators.md)
- compatible object lifetimes
- [compatible type layouts](0003-type-layout.md), or marshalling data using FFI glue

String APIs use vectors and iterators, so they inherit most vector and iterator interoperability problems.

This problem will also benefit from:

- [Function overloading in FFI bindings project goal](https://rust-lang.github.io/rust-project-goals/2026/overloading-for-ffi.html)

TODO: fill in the remainder of this section

## Acceptance Criteria
[acceptance-criteria]: #acceptance-criteria

Solutions should also handle (or explain why handling these types isn't needed):

- Rust/C++ string mutation
- Rust `OsString`
- Rust `CString` (potentially trivially)
- C/C++ `char *`, including `const`, `signed` and `unsigned`
- C/C++ `int8_t *` used as strings, including unsigned
- C++ small string optimization (non-trivial moves), and other layout and allocation variations

For some Windows use cases, solutions may also need to handle:

- C/C++ UTF-16 formats (using wide character types)

TODO: fill in the remainder of this section

## Impact
[impact]: #impact

TODO

## Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

TODO

## Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

TODO

## Prior art
[prior-art]: #prior-art

Rust Language/Library features:

- [str_from_raw_parts](https://github.com/rust-lang/rust/issues/119206)
- [cstr_bytes](https://github.com/rust-lang/rust/issues/112115) iterator
- [bstr](https://github.com/rust-lang/rust/issues/134915), strings that aren't always UTF-8

Windows-Only:

- [str_from_utf16_endian](https://github.com/rust-lang/rust/issues/116258)
- [utf16_extra](https://github.com/rust-lang/rust/issues/94919) (surrogate detection)

## Further Background
[further-background]: #further-background

TODO

## Experts & Champions
[experts--champions]: #experts--champions

David Sankel: A canonical set of C++ representations of Rust types would be useful.

TODO: fill in the remainder of this section

## Unresolved questions
[unresolved-questions]: #unresolved-questions

TODO
