- Problem Name: type_layout
- Start Date: 2026-02-19
- Problem Statement PR: [rustfoundation/interop-initiative#1](https://github.com/rustfoundation/interop-initiative/pull/1)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)
- C++ Issue: [????/????#0000](https://github.com/????/????/pull/0000)

## Summary
[summary]: #summary

Rust and C++ code can be compiled with different ABIs (compilers, compiler settings, and architectures).
Different ABIs can lead to different type layouts in memory, including size, alignment, field offsets, and padding.

Using the same layout in Rust and C++ enables zero-overhead data access, which is important for performance-sensitive use cases.
Mismatching layouts can lead to unsoundness, if data is not marshalled using the correct FFI glue code.

## Related Problems
[related-problems]: #related-problems

This problem is related to:

- cross-language build coordination

*TODO:* fill in the remainder of this section, and the rest of the template (during the detail phase)

Does this problem depend on any other problems being solved first? Do other problems depend on it?
Is it part of a larger group of problems?
Does solving this problem conflict with other problems?
