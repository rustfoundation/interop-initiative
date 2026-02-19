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

## Related Problems
[related-problems]: #related-problems

This problem is related to:

- cross-language build coordination

*TODO:* fill in the remainder of the template (during the detail phase)

Does this problem depend on any other problems being solved first? Do other problems depend on it?
Is it part of a larger group of problems?
Does solving this problem conflict with other problems?

<details>
<summary>template sections</summary>

## Impact
[impact]: #impact

Focus on clearly defining the Rust and C++ interoperability problem, and its impact on developers.
It should be factual, informative, and uncontroversial, supported by expert opinion.

This section should explain the problem in detail, including necessary background.
It should also contain several specific use cases where users encounter this problem.

This section is one of the most important sections of the problem statement, and can be lengthy.

It should *not* propose specific solutions, that is the role of Rust / C++ change proposals.

## Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

Explain the problem as it is typically encountered by Rust / C++ programmers. That generally means:

- Introducing existing language, library, or toolchain concepts.
- Explaining the problem largely in terms of examples.
- Explaining how Rust and C++ programmers should *think* about the problem. It should explain the impact as concretely as possible.
- If applicable, provide sample language, tool or execution error messages.

This section should focus on how compiler contributors should think about the problem, and give examples of its concrete impact.

## Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

This is the technical portion of the problem statement. Explain the problem in sufficient detail that:

- Its interactions with other interoperability problems and existing language/library features is clear.
- Corner cases are dissected by example.
- Performance, compatibility, and usability issues are highlighted where relevant.

The section should return to the examples given in the previous section, and explain more fully how the problem manifests in each example.

## Acceptance Criteria
[acceptance-criteria]: #acceptance-criteria

List the necessary and desirable solution constraints, without committing to any one solution.
If possible, they should be categorised by language, library, or tooling, and also Rust, C++, or both languages.

Is are there any solution properties that are out of scope?
Which residual issues are acceptable after the problem is solved?

## Prior art
[prior-art]: #prior-art

Discuss prior art in relation to this problem.
A few examples of what this can include are:

- What previous solutions have been tried for this Rust / C++ problem?
- Does a similar interoperability problem exist in other programming languages, and how have they tried to fix it?

## Further Background
[further-background]: #further-background

- Is there any other useful background information, that will help understand the problem?

## Experts & Champions
[experts--champions]: #experts--champions

Which experts understand the fine details of this problem?
Who is able to drive solutions on the Rust and C++ sides? (This can be an individual or a team.)

Link to documents and discussions that demonstrate community consensus on the problem: that it is current, important, and practical.

## Unresolved questions
[unresolved-questions]: #unresolved-questions

- What parts of the problem do you expect to resolve later?
- What related issues are out of scope for this problem statement?

</details>
