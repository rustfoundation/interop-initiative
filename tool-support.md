# Rust/C++ Interop Tool Support

***This document is an initial, partially reviewed draft***

This table describes how various Rust/C++ interop use cases and problems are supported by major
interop tooling, including the Rust compiler:
* [Crubit][crubit] auto-generates a wide range of supported Rust and C++ interop bindings. Glue code and customisation is in the original source code, and the build system. Currently depends on the bazel build system, but [they are actively working on `cargo` integration][crubit-cargo].
* [cxx]: auto-generates supported safe Rust and C++ interop bindings. Glue code is in the original source code, customisation and recent language features are limited. Compatible with [multiple build systems][cxx-multi-build], including `cargo`.
* [Zngur][zngur]: auto-generates a wide range of declared Rust and C++ interop bindings, focusing on accessing Rust from C++. Glue code is in C++, as the more expressive language. Declarations and customisation are in `.zng` [IDL] files. Compatible with multiple build systems.

[rustc]: https://rust-lang.org
[crubit]: https://crubit.rs
[cxx]: https://cxx.rs
[zngur]: https://hkalbasi.github.io/zngur/
[crubit-cargo]: https://crubit.rs/overview/status.html#usage-outside-of-google
[cxx-multi-build]: https://cxx.rs/building.html
[IDL]: https://en.wikipedia.org/wiki/Interface_description_language

*Key:*<br/>
⛔ No Support<br/>
🔁 In progress<br/>
◓ Basic or Partial Support<br/>
✅ Good Support<br/>
? Needs Analysis<br/>
*blank* Out of Scope<br/>

| Use Case or Problem Statement                          | [rustc]  | [Crubit] | [cxx] | [Zngur] |
| :----------------------------------------------------- | :------- | :------- | :---- | :---- |
| [Async C++/Rust Interop][async-interop]                | ⛔       | ✅       | ⛔    | ⛔    |
| *Basic Types*                                          | [rustc]  | [Crubit] | [cxx] | [Zngur] |
| [Compatible type layouts][type-layouts]                | ◓        | ✅       | ✅    | ✅    |
| [Match standard library type layouts][stdlib-layout]   | ◓        | ◓        | ◓     | ⛔    |
| [Create a vector containing FFI types][cross-vec]      |          | ✅       | ⛔    | ✅    |
| [Call C++ Iterators from Rust][cross-iter]             |          | ✅       | ✅    | ✅    |
| [Use FFI string types][string-ffi]                     |          | ✅       | ✅    | ?     |
| [Lossless FFI integer type mappings][ffi-int-1to1]     | ◓        | ✅[^5]   |       |       |
| [Define a C++ enum from Rust][c-enum-from-rust]        | ◓        | ✅       | ✅    | ⛔    |
| [Convert Rust & C++ Result types][cross-result-types]  |          | ✅       | ✅    |       |
| [Unify C++ forward decls to Rust types][cpp-fwd-decls] |          | ◓        | ◓     |       |
| *Advanced Types*                                       | [rustc]  | [Crubit] | [cxx] | [Zngur] |
| [Init C++ objects in-place in Rust][rust-inplace-init] | 🔁[^1]   | ✅       | ⛔    | ⛔    |
| [Passing non-POD types][non-pod-passing]               | ◓        | ✅       | ✅    | ?     |
| [Use a prefix of the full Rust/C++ type][type-prefix]  | 🔁[^4]   | ✅       | ✅    | ?     |
| [Rust Traits for C++ Types][traits-cpp-types]          | ◓        | ✅       | ◓?    | ✅    |
| [Rust dyn traits from C++][dyn-traits-cpp]             | ?        | ◓        | ⛔    | ✅    |
| [Cross-language class inheritance][cross-inherit]      |          | ◓        | ⛔    | ✅    |
| [C++ templates with Rust types][cpp-templ-rust]        |          | ✅       | ⛔    | ⛔    |
| *Safety*                                               | [rustc]  | [Crubit] | [cxx] | [Zngur] |
| [Cross-language object ownership][cross-own]           |          | ✅       | ◓     | ✅    |
| [Lifetimes in C++ code][cpp-lifetimes]                 |          | ◓        | ◓     |       |
| [FFI thread safety][ffi-thread-safe]                   | ◓        | ◓        | ?     | ?     |
| [Rust & C++ unwinding compatibility][ffi-unwind]       | ◓        | ⛔       | ✅    | ✅    |
| [Skip non-null pointer null checks][skip-null-checks]  | ?        | ✅       | ⛔    | ◓     |
| [Allocator FFI compatibility][ffi-alloc]               | 🔁[^6]   | ✅       | ✅    | ✅    |
| *Overloading*                                          | [rustc]  | [Crubit] | [cxx] | [Zngur] |
| [Call an overloaded C++ function from Rust][overload]  | 🔁[^3]   | ◓        | ◓     | ?     |
| [Type-dependent C++ call safety][cpp-type-dep-safe]    | 🔁[^2]   | ?        | ⛔    |       |
| *Building & Linking*                                   | [rustc]  | [Crubit] | [cxx] | [Zngur] |
| [Add Rust to existing C/C++ builds][rust-to-cpp-build] | ◓[^7]    | ◓        | ✅    | ✅    |
| [Link Time Optimisation (LTO) Interop][lto-interop]    | ◓        | ✅       | ✅    | ?     |

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
