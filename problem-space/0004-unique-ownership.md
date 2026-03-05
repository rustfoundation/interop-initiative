- Problem Name: unique_ownership
- Start Date: 2026-02-19
- Problem Statement PR: [rustfoundation/interop-initiative#1](https://github.com/rustfoundation/interop-initiative/pull/1)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)
- C++ Issue: [????/????#0000](https://github.com/????/????/pull/0000)

## Summary
[summary]: #summary

Rust and C++ have different unique ownership concepts.
In Rust, unique ownership is enforced by the compiler on most non-reference types, but there are multiple ways to opt-out (including `Copy` and various `unsafe` APIs).
In C++, ownership can be managed using `unique_ptr`, but significant amounts of code handle ownership implicitly.

Passing a C++ `unique_ptr` to Rust, or obtaining one from an owned Rust value, needs to handle the semantic and API differences between the languages.
For example, Zngur does this using `Ref` and `RefMut` types.

Many C++ types can't be put on the Rust stack, because C++ doesn't support trivial relocatability, but Rust assumes all types are trivially relocatable.
Fixing this might require C++ language changes, because using non-relocatable wrapper types in Rust significantly impacts ergonomics.

Some users might benefit from custom move operation support in Rust.

### Example Code
[example-code]: #example-code

This C++ code passes some read-write C++ string objects to Rust.
But as soon as Rust moves the C++ strings, it is undefined behaviour, because `string` is not trivially relocatable.

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

        void swap_strings_in_rust(std::basic_string<char> *cppstr, std::basic_string<char> *other);
    }

    extern "C" void pass_strings_to_cplusplus(const char *cstr, const char *cother) {
        auto cppstr = std::string(cstr, strlen(cstr));
        auto other = std::string(cother, strlen(cother));

        // Required for FFI safety by Rust (fails at compile time)
        //static_assert(std::is_trivially_move_constructible_v<typeof(cppstr)>);
        if (SIZE_OF_CPP_STRING != sizeof(cppstr)) { std::cout << "Size is " << SIZE_OF_CPP_STRING << " in Rust but " << sizeof(cppstr) << " in C++" << std::endl; abort(); }

        std::cout << cppstr << " | " << &cppstr << " | " << (void *)cppstr.c_str() << ", " << other << " | " << &other << " | " << (void *)other.c_str() << " before" << std::endl;
        swap_strings_in_rust(&cppstr, &other);
        std::cout << cppstr << " | " << &cppstr << " | " << (void *)cppstr.c_str() << ", " << other << " | " << &other << " | " << (void *)other.c_str() << " after" << std::endl;
    }
}}

#[unsafe(no_mangle)]
static SIZE_OF_CPP_STRING: usize = 24;

#[repr(C)]
struct CppString {
    data: [u8; SIZE_OF_CPP_STRING],
}

#[unsafe(no_mangle)]
unsafe fn swap_strings_in_rust(cppstr: &mut CppString, other: &mut CppString) {
    // Undefined behaviour: `string` is not trivially move constructible in C++
    std::mem::swap(cppstr, other);
}
```

## Related Problems
[related-problems]: #related-problems

Unique ownership is related to:

- shared and exclusive references
- [correct memory allocation and deallocation](0001-incompatible-allocators.md)
- [compatible type layouts](0003-type-layout.md), or marshalling data using FFI glue

This problem also depends on:

- [Beyond the `&` project goal](https://rust-lang.github.io/rust-project-goals/2026/roadmap-beyond-the-ampersand.html)
  - [Reborrow traits](https://rust-lang.github.io/rust-project-goals/2026/reborrow-traits.html)
  - [Field projections](https://rust-lang.github.io/rust-project-goals/2026/field-projections.html)
  - [In-place initialization](https://rust-lang.github.io/rust-project-goals/2026/in-place-init.html)
- [Immovable types & guaranteed destructors project goal](https://rust-lang.github.io/rust-project-goals/2026/move-trait.html)

And some use cases could benefit from:

- [Ergonomic ref-counting project goal](https://rust-lang.github.io/rust-project-goals/2026/ergonomic-rc.html)
- [specialization project goal](https://rust-lang.github.io/rust-project-goals/2026/specialization.html)

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

Rust Language/Library features:

Reference Ergonomics:

- [arbitrary_self_types](https://doc.rust-lang.org/nightly/unstable-book/language-features/arbitrary-self-types.html)
- [arbitrary_self_types_pointers](https://doc.rust-lang.org/nightly/unstable-book/language-features/arbitrary-self-types-pointers.html)
- [deref_patterns](https://github.com/rust-lang/rust/issues/87121)
- [pin_ergonomics](https://doc.rust-lang.org/nightly/unstable-book/language-features/pin-ergonomics.html)
- [reborrow](https://github.com/rust-lang/rust/issues/145612)

Constructor / Call Ergonomics:

- [super_let](https://doc.rust-lang.org/nightly/unstable-book/language-features/super-let.html)
- [fn_traits](https://doc.rust-lang.org/nightly/unstable-book/library-features/fn-traits.html), to do fixups before calling with a foreign type
  - [unboxed_closures](https://doc.rust-lang.org/nightly/unstable-book/language-features/unboxed-closures.html)

Safety:

- [unsafe_pinned](https://github.com/rust-lang/rust/issues/125735)

Performance:

- [specialization](https://github.com/rust-lang/rust/issues/31844)

TODO: fill in the remainder of this section

## Further Background
[further-background]: #further-background

TODO

## Experts & Champions
[experts--champions]: #experts--champions

David Sankel: Putting C++ types on the Rust stack is very important.

TODO: fill in the remainder of this section

## Unresolved questions
[unresolved-questions]: #unresolved-questions

TODO
