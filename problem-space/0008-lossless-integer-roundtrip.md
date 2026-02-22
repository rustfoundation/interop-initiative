- Problem Name: lossless_integer_roundtrip
- Start Date: 2026-02-20
- Problem Statement PR: [rustfoundation/interop-initiative#2](https://github.com/rustfoundation/interop-initiative/pull/2)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)
- C++ Issue: [????/????#0000](https://github.com/????/????/pull/0000)

## Summary
[summary]: #summary

Rust and C++ have different integer types. Some of these integer types have equivalents in the other language, but a few don't.

Many interop users would benefit from lossless roundtrips between Rust and C++. This might also improve performance in hot, integer-heavy code.

In Rust, the set of conversions available with `Into` is a subset(?) of lossless(?) `as` casts.

## Related Problems
[related-problems]: #related-problems

Fixing this might also benefit:

- [debugging](0006-debugging.md)
- [cross language link-time optimization](0007-cross-lang-lto.md)

*TODO:* fill in the remainder of this section

Is it part of a larger group of problems?
Does solving this problem conflict with other problems?

## Acceptance Criteria
[acceptance-criteria]: #acceptance-criteria

### Conversion Operations
[conversion-operations]: #conversion-operations

Rust should provide the following integer conversion operations:

- `.cast_sign()`
- `.extend()`
- `.truncate()`

Preferably all const-compatible.

### Currently Lossy Types
[currently-lossy-types]: #currently-lossy-types

TODO: double-check this categorisation

Solutions should handle (or explain why handling these types isn't needed):

- Rust's `std::ffi` uses `i8` for [`c_char`](https://doc.rust-lang.org/stable/std/ffi/type.c_char.html), but Crubit uses [`u8`](https://docs.rs/ffi_11/latest/ffi_11/struct.c_char.html)
- Requiring C++ `uint8_t` to be `unsigned char` (?)
- TODO: add extra examples here

On Windows:

- 32-bit `long` on 64-bit Windows (`signed` and `unsigned`)

Other integer types are already lossless.

### Out of Scope
[out-of-scope]: #out-of-scope

These types might not be relevant to interop:

- C++ `std::byte`, which is similar to `MaybeUninit<u8>`
- GCC targets where `size_t` is `unsigned __int20` and `int` is `i32` (not supported by Rust)

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
