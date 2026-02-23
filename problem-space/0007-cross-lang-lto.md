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
This is a bug in the code with undefined behaviour, rather than the compilers.

### Example Code
[example-code]: #example-code

TODO

## Related Problems
[related-problems]: #related-problems

Related to:

- [accurate information about type layouts](0003-type-layout.md)

These issues also impact non-interop use cases, like:

- the GCC-driven frontends and backends: `gcc-rs` and `rustc_codegen_gcc`

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
