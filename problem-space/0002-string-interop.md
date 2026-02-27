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

This Rust code passes a read-only Rust string pointer to C++:

```rust
use std::ffi::CStr;

unsafe extern "C" {
    fn use_string_in_cplusplus(pointer: *const u8, length: usize);
}

unsafe fn pass_string_to_cplusplus(cstr: &'static CStr) {
    let length = cstr.count_bytes();
    let pointer = cstr.as_ptr();
    // SAFETY:
    // - The pointer must not be read after Rust has deallocated or moved the string, or beyond `length + 1` (the `nul` byte).
    // - The pointer must never be written to.
    unsafe { use_string_in_cplusplus(pointer, length); };
}
```

The C++ code constructs a `std::string` by copying `length` bytes from the Rust pointer into a new allocation, which is an unacceptable performance trade-off for some use cases:

```c++
#include <cstdint>
#include <string>

void use_string_in_cplusplus(const uint8_t* pointer, size_t length) {
    auto cppstr = std::string((const char *)pointer, length);

    // do_something_with(cppstr);
}
```

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
