- Problem Name: cross_lang_lto
- Start Date: 2026-02-20
- Problem Statement PR: [rustfoundation/interop-initiative#2](https://github.com/rustfoundation/interop-initiative/pull/2)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)
- C++ Issue: [????/????#0000](https://github.com/????/????/pull/0000)

## Summary
[summary]: #summary

Link-time optimisation (LTO) allows code from different translation units to be optimised during linking into the final binary.
Theoretically, LTO should allow Rust and C++ code to be optimised together during linking.

However, current LTO implementations [add new miscompilations](https://rust-lang.zulipchat.com/#narrow/channel/136281-t-opsem/topic/How.20does.20the.20GCC.20backend.20handle.20code.20C.20would.20disallow.3F/near/394411414).

LTO needs to fully follow the semantics of Rust and C++ during linking, but how do we handle conflicting semantics?
For example:

- [clobbering neighboring padding](https://rust-lang.zulipchat.com/#narrow/channel/136281-t-opsem/topic/How.20does.20the.20GCC.20backend.20handle.20code.20C.20would.20disallow.3F/near/394120185) of an assigned field
- lifetime end zapping
- [preserving partially uninitialized data](https://rust-lang.zulipchat.com/#narrow/channel/136281-t-opsem/topic/How.20does.20the.20GCC.20backend.20handle.20code.20C.20would.20disallow.3F/near/394338200) on move
- [wrapping integer arithmetic](https://rust-lang.zulipchat.com/#narrow/channel/136281-t-opsem/topic/How.20does.20the.20GCC.20backend.20handle.20code.20C.20would.20disallow.3F/near/394411414)

Currently, the FFI boundary stops compilers taking advantage of some undefined behaviour (that is, it is not detected, so it doesn't cause any issues).
During LTO, this existing undefined behaviour [might be triggered](https://arxiv.org/pdf/2404.11671).
This is technically a bug in the code with undefined behaviour, rather than the compilers, but tools can assist developers to detect and avoid issues.

### Example Code
[example-code]: #example-code

This unsafe Rust code is well-defined, and prints `1, 0, 0, 0`:

```rust
use std::ptr::addr_of_mut;

#[repr(C)]
struct S(u8, u16);

fn main() {
    let mut mem: [u8; 4] = [0; 4];
    let p = addr_of_mut!(mem);
    let q: *mut S = p.cast();
    unsafe {
        (*q).0 = 1;
    }
    dbg!(mem);
}
```

The equivalent C++ code is not well-defined, because `q->byte = 1` can overwrite padding bytes.
It might print `1, 0, 0, 0,` on some compilers, but this is not guaranteed.

```c++
#include <cstdint>
#include <iostream>

typedef struct {
    uint8_t byte;
    uint16_t twobyte;
} s_t;

int main(void) {
    uint8_t mem[4] = { 0, 0, 0, 0 };
    uint8_t *p = mem;
    s_t *q = (s_t *)p;
    q->byte = 1;
    for (int i = 0; i < 4; i++) {
        std::cout << int(mem[i]) << ", ";
    }
}
```

For example, the following compilers produce different output:

- `0, 0, 0, 0,`: [x86-64 icx 2025.3.1, but only with `-O{,2,3,s,z}`](https://godbolt.org/z/4sa9Wrsz3)

This ill-defined C++ code can cause issues if the struct definitions, reads, or writes are in different languages, then linked using cross-language LTO.
Which language's semantics are followed?

Credit to [Jake Degen for these code examples](https://rust-lang.zulipchat.com/#narrow/channel/136281-t-opsem/topic/How.20does.20the.20GCC.20backend.20handle.20code.20C.20would.20disallow.3F/near/394120185).

## Related Problems
[related-problems]: #related-problems

Related to:

- [accurate information about type layouts](0003-type-layout.md)

These issues also impact non-interop use cases, like:

- the GCC-driven frontends and backends: `gcc-rs` and `rustc_codegen_gcc`

TODO: fill in the remainder of this section

## Impact
[impact]: #impact

One large project uses cross-language LTO with the same version of LLVM in `rustc` and `clang`.
has only caused one crash across millions of lines of code, and years of widespread operation.
This was due to a `cxx` bug which did not mark a C++ type as having interior mutability in Rust.

TODO: fill in the remainder of this section

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

Compiler arguments:

- The [-Zvirtual-function-elimination -Clto](https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/virtual-function-elimination.html)
options run unused Rust virtual function elimination passes during LTO. But this analysis has known miscompilations.

Rust Language/Library features:

- [link_arg_attribute](https://doc.rust-lang.org/nightly/unstable-book/language-features/link-arg-attribute.html)
- [used_with_arg](https://github.com/rust-lang/rust/issues/93798)

TODO: fill in the remainder of this section

## Further Background
[further-background]: #further-background

TODO

## Experts & Champions
[experts--champions]: #experts--champions

David Sankel: cross-language LTO is an important problem to get solved
David Tolnay: cxx supports cross-language LTO with the same version of LLVM in `rustc` and `clang`

TODO: fill in the remainder of this section

## Unresolved questions
[unresolved-questions]: #unresolved-questions

How much does this actually matter in practice?
That is, does the undefined behaviour cause actual issues at runtime?

TODO: fill in the remainder of this section
