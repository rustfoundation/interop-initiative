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

## Related Problems
[related-problems]: #related-problems

Creating and modifying strings at runtime depends on:

- [correct memory allocation and deallocation](0001-incompatible-allocators.md)
- [compatible type layouts](0003-type-layout.md), or marshalling data using FFI glue

String APIs use vectors and iterators, so they inherit most vector and iterator interoperability problems.

*TODO:* fill in the remainder of this section

Is it part of a larger group of problems?
Does solving this problem conflict with other problems?

## Acceptance Criteria
[acceptance-criteria]: #acceptance-criteria

Solutions should also handle (or explain why handling these types isn't needed):

- Rust `OsString`
- Rust `CString` (potentially trivially)
- C/C++ `char *`, including `const`, `signed` and `unsigned`
- C/C++ `int8_t *` used as strings, including unsigned
- C++ small string optimization (non-trivial moves), and other layout and allocation variations

For some Windows use cases, solutions may also need to handle:

- C/C++ UTF-16 formats (using wide character types)

*TODO:* fill in the rest of the template (during the detail phase)
