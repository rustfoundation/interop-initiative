- Problem Name: lossless_integer_roundtrip
- Start Date: 2026-02-20
- Problem Statement PR: [rustfoundation/interop-initiative#2](https://github.com/rustfoundation/interop-initiative/pull/2)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)
- C++ Issue: [????/????#0000](https://github.com/????/????/pull/0000)

## Summary
[summary]: #summary

Rust and C++ have different integer types. Some of these integer types have equivalents in the other language, but a few don't.

Many interop users would benefit from lossless roundtrips between Rust and C++. This might also improve performance in hot, integer-heavy code.

Rust's FFI integer types are based on the most popular C compiler (and compiler settings) on each architecture, which causes challenges for less popular compilers and non-standard settings.

In Rust, the set of conversions available with `Into` is a subset of lossless `as` casts.
The `as` operator can also silently perform lossy casts in Rust, but there are `clippy` lints that:

- [convert lossless `as` casts to `Into`](https://rust-lang.github.io/rust-clippy/stable/index.html#cast_lossless)
- detect and prevent [truncating](https://rust-lang.github.io/rust-clippy/stable/index.html#cast_possible_truncation), [wrapping](https://rust-lang.github.io/rust-clippy/stable/index.html#cast_possible_wrap), and [sign loss](https://rust-lang.github.io/rust-clippy/stable/index.html#cast_sign_loss) `as` casts
- [avoid redundant casts](https://rust-lang.github.io/rust-clippy/stable/index.html#unnecessary_cast)

### Example Code
[example-code]: #example-code

The following Rust integer and floating point types are guaranteed to match the corresponding C type, for the most popular compiler on each platform (as defined by Rust when that target was added). These C types can also be used in C++.

| Rust          | C                 | Notes                                         |
| ------------- | ----------------- | --------------------------------------------- |
| u{N}          | uint{N}_t         | N = 8, 16, 32, 64                             |
| i{N}          | int{N}_t          | N = 8, 16, 32, 64                             |
| u128          | __uint128_t       | [1] [2]                                       |
| i128          | __int128_t        | [1] [2]                                       |
| f{N}          | boost::float{N}_t | [1], [3], N = 16, 32, 64, 128                 |

[1]: C availability is platform dependent
[2]: C name is platform dependent
[3]: Not available in C or C++ standard libraries

The following Rust FFI types are guaranteed to match the corresponding C type, for the most popular compiler on each platform (as defined by Rust when that target was added).
These C types can also be used in C++.

| Rust core::ffi:: | C             | Notes                                                       |
| ---------------- | ------------- | ----------------------------------------------------------- |
| c_{type}         | {type}        | {type} names are in the [Rust core library docs][ffi-types] |
| *mut c_void      | void *        | Only equivalent when used as a pointer                      |
| *const c_void    | const void *  | Only equivalent when used as a pointer                      |

[ffi-types]: https://doc.rust-lang.org/core/ffi/index.html#types

The following Rust "integer" types have no corresponding C or C++ types:

| Rust | Notes                                                 |
| ---- | ----------------------------------------------------- |
| bool | 8 bits, but the only valid values are 0 and 1         |
| char | A Unicode scalar value, surrogates are invalid values |

The following C/C++ integer types have no corresponding Rust type:

| C/C++              | Notes                                                |
| ------------------ | ---------------------------------------------------- |
| bool               | size is implementation-defined                       |
| unsigned field:{N} | Only compatible with u{N} for N = 8, 16, 32, 64, 128 |

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
