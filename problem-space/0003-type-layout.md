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

### Example Code
[example-code]: #example-code

This Rust code passes a read-only Rust struct to C++:

```rust
#[repr(C)]
struct Example {
    byte: u8,
    fractional: f32,
    integer: u128,
}

unsafe extern "C" {
    fn use_struct_in_cplusplus(pointer: *const Example);
}

unsafe fn pass_struct_to_cplusplus(example: &'static Example) {
    let pointer =std::ptr::from_ref(example);
    // SAFETY:
    // - The pointer must not be read after Rust has deallocated or moved the struct, or beyond the size of the struct.
    // - The pointer must never be written to.
    unsafe { use_struct_in_cplusplus(pointer); };
}
```

The C++ code has its own definition of the struct, which must be kept ABI-compatible with the Rust struct `Example`.
This is particularly tricky in the presence of non-standard types, such as floating point and 128-bit integers.

```c++
#include <cstdint>
#include <boost/cstdfloat.hpp>

typedef struct {
    uint8_t byte;
    boost::float32_t fractional,
    __uint128_t integer;
} example_t;

void use_struct_in_cplusplus(const example_t* pointer) {
    // do_something_with(pointer);
}
```

See also the "Layout issues" section of [RFC 3845](https://github.com/rust-lang/rfcs/pull/3845/changes), which has multiple platform-specific Rust/C layout compatibility code examples.

## Related Problems
[related-problems]: #related-problems

This problem is related to:

- cross-language build coordination
- [The crABI experiment](https://github.com/rust-lang/rust/pull/105586)

Solving this problem on the Rust side depends on parts of:

- [repr(C) and ordered fields confusion](https://github.com/rust-lang/rfcs/pull/3845)

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

## Acceptance Criteria
[acceptance-criteria]: #acceptance-criteria

TODO

## Prior art
[prior-art]: #prior-art

TODO

## Further Background
[further-background]: #further-background

TODO

## Experts & Champions
[experts--champions]: #experts--champions

TODO

## Unresolved questions
[unresolved-questions]: #unresolved-questions

TODO
