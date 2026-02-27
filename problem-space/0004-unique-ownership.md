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

This C++ code passes a read-write C++ string object to Rust:

```c++
#include <string>
#include <cstring>
#include <cassert>

extern "C" {
    extern const size_t SIZE_OF_CPP_STRING;

    void use_string_in_rust(std::basic_string<char> cppstr);
}

void pass_string_to_rust(const char *cstr) {
    auto cppstr = std::string(cstr, strlen(cstr));

    // Required for FFI safety by Rust, but fails at compile time
    //static_assert(std::is_trivially_move_constructible_v<typeof(cppstr)>);
    assert(sizeof(cppstr) == SIZE_OF_CPP_STRING);

    use_string_in_rust(cppstr);
}
```

But as soon as Rust moves the C++ string, it is undefined behaviour:

```rust
static SIZE_OF_CPP_STRING: usize = 32;

#[repr(C)]
struct CppString {
    data: [u8; SIZE_OF_CPP_STRING],
}

unsafe fn use_string_in_rust(cppstr: CppString) {
    // Undefined behaviour: `string` is not trivially move constructible in C++
    let moved_cppstr = cppstr;
}
```

## Related Problems
[related-problems]: #related-problems

Ownership is related to:

- shared and exclusive references
- [correct memory allocation and deallocation](0001-incompatible-allocators.md)
- [compatible type layouts](0003-type-layout.md), or marshalling data using FFI glue

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
