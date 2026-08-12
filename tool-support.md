# Rust/C++ Interop Tool Support

***This document is an initial, un-reviewed draft***

This table describes how various Rust/C++ interop use cases and problems are supported by major
interop tooling, including the Rust compiler.

*Key:*<br/>
⛔ No Support<br/>
🔁 In progress<br/>
◓ Basic Support<br/>
✅ Good Support<br/>
? Needs Analysis<br/>
*blank* Out of Scope<br/>

| Use Case or Problem Statement                          | rustc    | Crubit   | cxx   | Zngur |
| :----------------------------------------------------- | :------- | :------- | :---- | :---- |
| [Rust Traits for C++ Types][traits-cpp-types]          | ◓        | ✅       | ✅    | ✅    |
| [Rust dyn traits from C++][dyn-traits-cpp]             | ?        | ?        | ⛔    | ✅    |
| [Async C++/Rust Interop][async-interop]                | ⛔       | ⛔       | ⛔    | ⛔    |
| [Define a C++ enum from Rust][c-enum-from-rust]        | ◓        | ✅       | ✅    | ⛔    |
| [Match standard library type layouts][stdlib-layout]   | ◓        | ✅       | ✅    | ⛔    |
| [Link Time Optimisation (LTO) Interop][lto-interop]    | ◓        | ✅       | ✅    | ⛔    |
| [Init C++ objects in-place in Rust][rust-inplace-init] | 🔁[^1]   | ✅       | ⛔    | ⛔    |
| [Skip non-null pointer null checks][skip-null-checks]  | ?        | ✅       | ⛔    | ⛔    |
| [Lifetimes in C++ code][cpp-lifetimes]                 |          | ✅       | ✅    | ⛔    |
| [Unify C++ forward decls to Rust types][cpp-fwd-decls] |          | ✅       | ✅    | ⛔    |
| [Type-dependent C++ call safety][cpp-type-dep-safe]    | 🔁[^2]   | ?        | ⛔    | ⛔    |
| [C++ templates with Rust types][cpp-templ-rust]        |          | ✅       | ⛔    | ⛔    |
| [Call an overloaded C++ function from Rust][overload]  | 🔁[^3]   | ◓        | ◓     | ?     |
| [Use a prefix of the full Rust/C++ type][type-prefix]  | 🔁[^4]   | ✅       | ✅    |       |
| [Cross-language class inheritance][cross-inherit]      |          | ◓       | ⛔    | ✅    |
| [Create a vector containing FFI types][cross-vec]      |          | ✅       | ⛔    | ✅    |
| [Use FFI string types][string-ffi]                     |          | ✅       | ✅    | ?     |
| [Lossless FFI integer type mappings][ffi-int-1to1]     | ◓        | ✅[^5]   |       |       |
| [FFI thread safety][ffi-thread-safe]                   | ◓        | ?        | ?     | ?     |
| [Rust & C++ unwinding compatibility][ffi-unwind]       | ◓        | ⛔       | ✅    | ✅    |
| [Allocator FFI compatibility][ffi-alloc]               | 🔁[^6]   | ✅       | ✅    | ⛔    |
| [Passing non-POD types][non-pod-passing]               | ◓        | ✅       | ✅    | ?     |
| *Broad Support*                                        | rustc    | Crubit   | cxx   | Zngur |
| [Cross-language object ownership][cross-own]           |          | ✅       | ✅    | ✅    |
| [Add Rust to existing C/C++ builds][rust-to-cpp-build] | ◓[^7]    | ✅       | ✅    | ✅    |
| [Convert Rust & C++ Result types][cross-result-types]  |          | ✅       | ✅    | ✅    |
| [Call C++ Iterators from Rust][cross-iter]             |          | ✅       | ✅    | ✅    |
| [Compatible type layouts][type-layouts]                | ◓        | ✅       | ✅    | ✅    |

[traits-cpp-types]: https://github.com/rustfoundation/interop-initiative/issues/70
[dyn-traits-cpp]: https://github.com/rustfoundation/interop-initiative/issues/69
[async-interop]: https://github.com/rustfoundation/interop-initiative/issues/71
[c-enum-from-rust]: https://github.com/rustfoundation/interop-initiative/issues/68
[stdlib-layout]: https://github.com/rustfoundation/interop-initiative/issues/67
[lto-interop]: https://github.com/rustfoundation/interop-initiative/issues/66
[rust-inplace-init]: https://github.com/rustfoundation/interop-initiative/issues/61
[skip-null-checks]: https://github.com/rustfoundation/interop-initiative/issues/50
[cpp-lifetimes]: https://github.com/rustfoundation/interop-initiative/issues/48
[cpp-fwd-decls]: https://github.com/rustfoundation/interop-initiative/issues/47
[cpp-type-dep-safe]: https://github.com/rustfoundation/interop-initiative/issues/32
[cpp-templ-rust]: https://github.com/rustfoundation/interop-initiative/issues/15
[overload]: https://github.com/rustfoundation/interop-initiative/issues/14
[type-prefix]: https://github.com/rustfoundation/interop-initiative/issues/11
[cross-inherit]: https://github.com/rustfoundation/interop-initiative/issues/9
[cross-vec]: https://github.com/rustfoundation/interop-initiative/issues/9
[string-ffi]: https://github.com/rustfoundation/interop-initiative/issues/7
[ffi-int-1to1]: https://github.com/rustfoundation/interop-initiative/issues/6
[ffi-thread-safe]: https://github.com/rustfoundation/interop-initiative/issues/73
[ffi-unwind]: https://github.com/rustfoundation/interop-initiative/issues/65
[ffi-alloc]: https://github.com/rustfoundation/interop-initiative/issues/62
[cross-own]: https://github.com/rustfoundation/interop-initiative/issues/64
[rust-to-cpp-build]: https://github.com/rustfoundation/interop-initiative/issues/13
[cross-result-types]: https://github.com/rustfoundation/interop-initiative/issues/49
[cross-iter]: https://github.com/rustfoundation/interop-initiative/issues/21
[type-layouts]: https://github.com/rustfoundation/interop-initiative/issues/63
[non-pod-passing]: https://github.com/rustfoundation/interop-initiative/issues/77

[^1]: [Rust project goal](https://rust-lang.github.io/rust-project-goals/2026/in-place-init.html)
[^2]: [Rust RFC](https://github.com/rust-lang/rfcs/pull/2375)
[^3]: [Rust compiler experiment](https://github.com/rust-lang/rust/issues/153629)
[^4]: [Rust compiler experiment](https://github.com/rust-lang/rust/issues/130494), requires a Rust `UnsafeCell` to wrap C++ mutable class members
[^5]: [`ffi_11` crate](https://crates.io/crates/ffi_11)
[^6]: [Rust Zulip discussions](https://rust-lang.zulipchat.com/#narrow/channel/197181-t-libs.2Fwg-allocators/topic/Near-term.20path.20to.20stabilizing.20an.20MVP.20of.20.60trait.20Allocator.60.3F/with/441587971)
[^7]: [Rust stabilisation RFC](https://github.com/rust-lang/rfcs/pull/3993)
