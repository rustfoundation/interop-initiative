- Problem Name: string_interop
- Start Date: 2026-02-18
- Problem Statement PR: [rustfoundation/interop-initiative#1](https://github.com/rustfoundation/interop-initiative/pull/1)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)
- C++ Issue: [????/????#0000](https://github.com/????/????/pull/0000)

## Summary
[summary]: #summary

Rust and C++ have different owned string and string slice types, `String`/`string` and `str`/`string_view`.

These types have different memory layouts, valid character constraints, operations, and operational semantics.
In both languages, type layouts are implementation-defined, and can vary based on the compiler, platform, and the origin of the string data (C++).

### C++ to Rust

Most Rust library APIs use the `String` or `&str` types, rather than traits abstracting string storage and operations.
Converting an arbitrary-length string for every C++ to Rust call is expensive. And the conversion may need to handle invalid characters.
Using `CString` or `OsString` avoids character validity issues, but significantly reduces API compatibility, because fewer APIs support those types.

### Rust to C++

On the C++ side, the situation is similar, many APIs take `string` or `string_view`, with fixed `char_traits`.
But other APIs take the `basic_string` or `basic_string_view` templates, which could technically wrap Rust UTF-8 strings.
However, maintaining consistent semantics across all string operations is challenging, particularly for writeable strings.

## Related Problems
[related-problems]: #related-problems

Creating and modifying strings at runtime depends on:

- [correct memory allocation and deallocation](0001-incompatible-allocators.md)
- [compatible type layouts](0003-type-layout.md), or marshalling data using FFI glue

String APIs use vectors and iterators, so they inherit most vector and iterator interoperability problems.

*TODO:* fill in the remainder of the template (during the detail phase)

<details>
<summary>TODO: expand related problems</summary>
Does this problem depend on any other problems being solved first? Do other problems depend on it?
Is it part of a larger group of problems?
Does solving this problem conflict with other problems?

</details>

## Acceptance Criteria
[acceptance-criteria]: #acceptance-criteria

Solutions should also handle:

- Rust `OsString`
- Rust `CString` (potentially trivially)
- C/C++ `char *`, including `const`, `signed` and `unsigned`
- C/C++ `int8_t *` used as strings, including unsigned
- C++ small string optimization (non-trivial moves), and other layout and allocation variations

For some Windows use cases, solutions may also need to handle:

- C/C++ UTF-16 formats (using wide character types)

<details>
<summary>TODO: expand acceptance criteria</summary>
List the necessary and desirable solution constraints, without committing to any one solution.
If possible, they should be categorised by language, library, or tooling, and also Rust, C++, or both languages.

Is are there any solution properties that are out of scope?
Which residual issues are acceptable after the problem is solved?
</details>

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
