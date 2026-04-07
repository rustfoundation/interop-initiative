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

This Rust code passes a read-only Rust struct to C++.

The C++ code has its own definition of the struct, which must be kept ABI-compatible with the Rust struct `Example`.
This is particularly tricky in the presence of non-standard types, such as floating point and 128-bit integers.

```rust
use cpp::cpp;

#[repr(C)]
#[derive(Debug)]
struct Example {
    byte: u8,
    fractional: f32,
    integer: u128,
}

fn main() {
    let ex = Example {
        byte: 42,
        fractional: 3.5,
        integer: 9000,
    };
    println!("{ex:?}");
    unsafe {
        pass_struct_to_cplusplus(&ex);
    }
}

unsafe fn pass_struct_to_cplusplus(ex: &Example) {
    let ex_ptr = std::ptr::from_ref(ex);
    // SAFETY:
    // - The pointer must not be read after Rust has deallocated or moved the struct, or beyond the size of the struct.
    // - The pointer must never be written to.
    unsafe {
        use_struct_in_cplusplus(ex_ptr);
    };
}

unsafe extern "C" {
    fn use_struct_in_cplusplus(ex_ptr: *const Example);
}

cpp! {{
    #include <iostream>
    #include <cstdint>
    //#include <boost/cstdfloat.hpp>
    namespace boost {
        typedef float float32_t;
    }

    typedef struct {
        uint8_t byte;
        boost::float32_t fractional;
        __uint128_t integer;
    } example_t;

    extern "C" void use_struct_in_cplusplus(const example_t* ex_ptr) {
        std::cout << "Example: " << ex_ptr->byte << ", " << ex_ptr->fractional << ", " << (unsigned long long)ex_ptr->integer << std::endl;
    }
}}
```

This code can be run using the [`cpp` crate](https://docs.rs/cpp/latest/cpp/macro.cpp.html), like the [Build and Run code example](https://github.com/rustfoundation/interop-initiative/issues/5).

See also the "Layout issues" section of [RFC 3845](https://github.com/rust-lang/rfcs/pull/3845/changes), which has multiple platform-specific Rust/C layout compatibility code examples.

## Related Problems
[related-problems]: #related-problems

This problem is related to:

- cross-language build coordination
- [The crABI experiment](https://github.com/rust-lang/rust/pull/105586)

Solving this problem on the Rust side depends on parts of:

- [repr(C) and ordered fields confusion](https://github.com/rust-lang/rfcs/pull/3845)
- [Unblocking dormant traits (extern types) project goal](https://rust-lang.github.io/rust-project-goals/2026/roadmap-unblocking-dormant-traits.html)
- [Expansion-time evaluation (compiler interop) project goal](https://rust-lang.github.io/rust-project-goals/2026/expansion-time-evaluation.html)
  - [Reflection and comptime](https://rust-lang.github.io/rust-project-goals/2026/reflection-and-comptime.html)
- [Open enums project goal](https://rust-lang.github.io/rust-project-goals/2026/open-enums.html)

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

Compatibility tooling:
- [Crubit's protobuf layouts are identical in Rust and C++](https://crubit.rs/types/protobuf.html)

Compiler arguments:

- The [-Zrandomize-layout](https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/randomize-layout.html)
compiler flag randomizes the layout of `repr(Rust)` types, but it does not cover all possible layout permutations.

Rust Language/Library features:

Layout & alignment:

- [layout_for_ptr](https://github.com/rust-lang/rust/issues/69835)
- [pointer_is_aligned_to](https://github.com/rust-lang/rust/issues/96284)
- [fn_align](https://github.com/rust-lang/rust/issues/82232)
- [static_align](https://github.com/rust-lang/rust/issues/146177)
- [sized_hierarchy](https://doc.rust-lang.org/nightly/unstable-book/language-features/sized-hierarchy.html)

Offsets:

- [offset_of_enum](https://doc.rust-lang.org/nightly/unstable-book/language-features/offset-of-enum.html)
- [offset_of_slice](https://doc.rust-lang.org/nightly/unstable-book/language-features/offset-of-slice.html)

Type compatibility:

- [c_size_t](https://github.com/rust-lang/rust/issues/88345)
- [thin_box](https://github.com/rust-lang/rust/issues/92791)

TODO: fill in the remainder of this section

## Further Background
[further-background]: #further-background

TODO

## Experts & Champions
[experts--champions]: #experts--champions

TODO

## Unresolved questions
[unresolved-questions]: #unresolved-questions

TODO
