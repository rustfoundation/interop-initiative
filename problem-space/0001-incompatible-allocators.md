- Problem Name: incompatible_allocators
- Start Date: 2026-02-18
- Problem Statement PR: [rustfoundation/interop-initiative#1](https://github.com/rustfoundation/interop-initiative/pull/1)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)
- C++ Issue: [????/????#0000](https://github.com/????/????/pull/0000)

## Summary
[summary]: #summary

Rust and C++ code can be compiled with different memory allocators (or allocation settings).
Allocating memory with one allocator, but deallocating it with an incompatible allocator, causes unsoundness.

This happens when ownership is passed from Rust to C++, then the memory is deallocated using C++ deallocation routines.
(Or the other way around.) Deallocation can happen explicitly using `drop`/`delete`, or implicitly at the end of a block.

FFI safety documentation often focuses on ensuring the same layout in Rust and C++. But deallocation also needs to be compatible for soundness.

### Example Code
[example-code]: #example-code

This Rust code passes ownership of a Rust heap-allocated array pointer, then tries to deallocate it using C++.
This is undefined behaviour, and in practice, it may attempt to deallocate the wrong amount of memory, or the wrong pointer.
(If the allocator prefix metadata is a different length.)

```rust
unsafe extern "C" {
    fn take_ownership_in_cplusplus(pointer: *mut u32, length: usize);
}

unsafe fn pass_ownership_to_cplusplus(list: Box<[u32; 7]>) {
    let pointer = Box::into_raw(list);
    // SAFETY:
    // - The layout is guaranteed to be `*mut [u32; 7]`, which is guaranteed to be `*mut u32`.
    // - C++ must deallocate this memory like Rust `Layout::array::<u32>(length)`.
    unsafe { take_ownership_in_cplusplus(pointer as *mut u32, 7); };
}
```

The actual deallocation length is taken from allocator metadata, which might not be at the same offset in different allocators.
(If the allocators are compatible, this might be defined behavior in practice.)

```c++
#include <cstddef>
#include <cstdint>
#include <cstdlib>

void take_ownership_in_cplusplus(uint32_t* pointer, size_t length) {
    // do_something_with(pointer, length);

    // Undefined behaviour: deallocates memory that was not allocated with `operator new`.
    // The length hint is ignored by default.
    ::operator delete(pointer, length * sizeof(*pointer));
}
```

The always-correct way to deallocate is to call:

```rust
unsafe fn pass_ownership_to_rust(pointer: *mut u32, length: usize) {
    assert_eq!(length, 7);
    // SAFETY:
    // - Allocated using Rust's Global allocator, using the same layout.
    let list = unsafe { Box::<[u32; 7]>::from_raw(pointer as *mut [u32; 7]) };
    std::mem::drop(list);
}
```

## Related Problems
[related-problems]: #related-problems

This problem is related to:

- `drop`/destructor mismatch, which can be a code correctness or a soundness issue
- [type layout](0003-type-layout.md): some allocator data structures use the type system

TODO: fill in the remainder of this section

## Impact
[impact]: #impact

TODO

## Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

TODO

Needs to cover (or declare out of scope) these alternative/edge cases:

```c++
    // Undefined behaviour: deallocates memory that was not allocated with malloc, calloc, or realloc.
    // Likely to deallocate the wrong amount of memory, because it doesn't know the length.
    free(pointer);

    // Undefined behaviour: deallocates memory that was not allocated with `operator new[]`.
    // Typically, reads `size_t` bytes before the allocation to determine the array size.
    // Likely to deallocate the wrong amount of memory (the length hint is ignored by default).
    ::operator[] delete(pointer, length * sizeof(*pointer));
```

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
