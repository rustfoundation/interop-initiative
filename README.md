# C++/Rust Interoperability Problem Statement

## The Initiative Vision

In collaboration with the Rust Foundation, Rust Project, and appropriate external stakeholders, make C++ and Rust interoperability easily accessible and approachable to the widest possible audience.

## Summary

Both C++ and Rust will play important roles in systems programming[^1] for the foreseeable future. With such a high degree of overlap in applicability, using both languages together is essential to pursuing safety and performance which is maintainable and scalable. Empowering technologists to choose the language best suited to their situation and minimizing the costs and risks of interoperation is the core goal of the initiative.

Despite C interoperability being a focus of Rust since its inception, various factors have inhibited a mature, standard and automatic solution for developing software using C++ and Rust together. To overcome these obstacles, the initiative will pursue a top-down, problem-space[^2] approach to facilitate cooperation and consensus among stakeholders including the Rust Project, the Rust Foundation member organizations, and the community of individuals and organizations using C++ or Rust. Together, [the material resources contributed to the initiative](https://foundation.rust-lang.org/news/google-contributes-1m-to-rust-foundation-to-support-c-rust-interop-initiative/) will be used to pursue three concurrent strategies:

1. Improve existing tools and address tactical issues within the Rust Project to reduce interoperability friction and risk in the short term
2. Build consensus around long-term goals requiring changes to Rust itself and develop the tactical approaches to begin pursuing them
3. Engage with the C++ community and committee to improve the quality of interoperation for both languages to help realize the mutual goals of safety and performance

The problem-oriented approach is essential to overcoming the obstacles in this space. Much work has been done on tools which provide considerable value[^3], but there is a limit to what can be achieved with external libraries and code-generation tools; successfully motivating deeper changes necessitates strategic consensus in order to define concrete tactical work. Consequently, this document does not define solutions; it serves as a call for input and participation in refining these strategies and the tactics that follow. The remainder of this document describes the problem itself and these strategies in greater detail.

[^1]: Used here in the sense of resource-constrained programming. In addition to operating systems and embedded programming, this includes applications such as games, databases, and web browsers.
[^2]: The problem-space is embodied by this document: describing the costs of the current situation and the desired properties of solutions without prescribing a particular solution. The top-down approach refers to engaging with leaders and stakeholders to craft the strategy, tactics and compromises for solutions which can best provide these properties.
[^3]: These tools broadly cover two areas of functionality: (1) binding generation to create appropriate types/signatures for the foreign symbols which will be linked against and (2) glue code and translation types to facilitate the interaction with data from the foreign side of the interface and improve ergonomics.

## The Problem

C++/Rust interoperability is the ability to exchange data and execute code written in both languages. There is an inherent trade-off between simplicity and efficiency and a distinct dichotomy between *inter*-process communication[^4] and *intra*-process interoperability. The latter occurs within the same executable, either through integrated compilation of multiple languages (inline embedding)[^5] or foreign function interfaces (FFI). The interop initiative is exclusively focused on the latter form, so all further mentions of interoperability in this document refer to *intra*-process interoperability. There are many open questions not covered here, such as which Rust and C++ language features can be effectively supported across FFI and how static vs dynamic linking affects interoperability.

[^4]: For example: files, sockets and shared memory. See https://en.wikipedia.org/wiki/Inter-process_communication
[^5]: This can also include cross-language, link-time optimization (x-lang LTO), which is a potential optimization for interop, but likely depends on the same IR being used to compile both languages.

### Who is impacted?

Though both Rust and C++ are used outside their primary domain as systems languages, this type of interoperability is particularly relevant to the systems domain where resources are constrained. The desire for interoperability depends on the particular system, but the common use cases are:

1. C++ systems adding or replacing functionality with Rust
2. Rust systems using existing C++ code
3. Polyglot systems supporting Rust (such as for plugin architectures)

Because of this dynamic and C++’s long history prior to Rust, the first case is by far the most common.

Since there are currently no toolchains[^6] which support mixing C++ and Rust in the same source file, the existing approaches to interoperability are focused on FFI-based solutions. FFI interoperation occurs via platform C ABIs today, because they are less platform-dependent and more stable than platform C++ ABIs. As such, the more C++-specific facilities are relied upon, the greater the challenge becomes in effectively translating them through a C-based interface.

[^6]: If such systems exist, they’re not widely available or practical for general use. Furthermore, the semantics of such a system would require significant definition at the language level. Proc macro crates exist for embedding C++ code in Rust, but have limited support for transferring values across the language boundary.

### When and where is this relevant?

As Rust approaches the 10th anniversary of its [1.0 stable release](https://blog.rust-lang.org/2015/05/15/Rust-1.0.html), it is transitioning from the “early adopters” stage to the “early majority” stage of the [technology adoption lifecycle](https://en.wikipedia.org/wiki/Technology_adoption_life_cycle). This transition aligns with a growing emphasis from [industry](https://msrc.microsoft.com/blog/2019/07/why-rust-for-safe-systems-programming/) and [government](https://www.whitehouse.gov/wp-content/uploads/2024/02/Final-ONCD-Technical-Report.pdf) on adopting memory-safe languages, with Rust standing out as a prime choice. Rust uniquely combines robust support for concurrency with guaranteed freedom from undefined behavior, making it a compelling alternative to C++ in environments where garbage collection is not feasible. Its proven reliability in these areas cements Rust as a critical tool for safer, high-performance systems.

While Rust has changed significantly in this period, the C++ interoperability story remains limited by C ABI underpinnings. A significant amount of development has gone into libraries to facilitate interoperability with both C and C++, but from the language and compiler level, the situation remains largely unchanged from the early days of Rust.

As the desire to integrate Rust into more C++ codebases increases, the value of making C++/Rust interoperability safer, easier, and more efficient is rapidly increasing. While each language takes a different overall approach, both view safety as an essential concern in modern systems. Both Rust and C++ have language- and standard-library-level facilities to improve safety in seemingly compatible ways, but significant benefits are lost when transiting the FFI boundary using the C ABI.

### What is the impact?

As a systems language, Rust was always intended to have interoperability with C. Even for non-systems languages, C is the _lingua franca_ for FFI generally and accessing OS-level resources in particular. As such, C ↔︎ Rust interoperability is *relatively* straightforward and the burden on the Rust programmer comes in two major forms:

1. Communication is limited to interfaces expressible in the C type system, precluding much of Rust’s ergonomic and safety benefits.
2. The FFI boundary itself is typically unsafe[^7], meaning guarantees such as freedom from undefined behavior and data races are lost.

The interoperability challenges Rust developers face are mirrored by C++ developers who also must sacrifice safety and ergonomic features not native to C. The superset[^8] relationship of C++ to C simplifies some of the translation of interface types, but all the beneficial aspects specific to C++ are lost. Even a codebase that expertly utilizes modern C++ features to improve safety and performance must introduce a region of more subtle and hazardous code around the point of FFI. This results in both additional developer effort and reduced quality of the codebase to support interoperability.

The consequence of this increased cost to interoperate means both C++ and Rust codebases are less able to access valuable code that already exists in the other language, and the ability to transition system components from one language to another is reduced outside of existing C-like interface boundaries. Ultimately, this reduction in freedom leads to worse outcomes for all users since technologists are less free to choose the most effective solutions.

[^7]: While external symbols [*can* be explicitly declared safe](https://rust-lang.github.io/rfcs/3484-unsafe-extern-blocks.html), this is uncommon both because the Rust compiler cannot verify the accuracy of externally declared symbols and because there is no ABI—apart from Rust’s native one—which provides an appropriate peer which is free from undefined behavior like default safe Rust.
[^8]: Modern C++ is no longer a perfect superset of modern C, but for practical interop purposes, this is effectively true.

### Why does this happen?

C has been around since the early ‘70s and C++ since the mid ‘80s. So why, four decades later, is there no better way to interoperate with C++? Partially, this is because all interoperability is a two-sided endeavor, so the quality of the interface depends on the languages on *both* sides. The interface must be defined in terms both languages can express[^9]. Whether those terms are native to the language (as C interfaces are to C++), can make the process more or less convenient, but the fundamental richness of the interface depends upon its capacity to express the semantics of each language’s constructs. Anything not natively representable within the interface must be communicated through a well-defined transformation to and from the interface[^10], but there must also be compatible constructs on either side. By definition, interoperation happens between languages that are *different*, so—similar to translation problems in natural languages—there will always be some amount of semantic loss through translation.

Why is the situation not already better? C++ was already quite mature by the time Rust was created, and Rust had its origins within Mozilla, alongside a multi-million line, decades-old C++ codebase with the explicit intention of being an alternative to C++. Apart from C, none of the major[^11] systems languages developed prior to C++ have seen significant modern use. Of those languages developed subsequent to C++, only a few have seen significant commercial use[^12]: Go, Rust and Swift. The C++ interoperability stories for all these languages is informative, but the intended use and governance of both Go and Swift are quite different from Rust. Perhaps most significant is the social piece: though initially developed within Mozilla research, the Rust Project has been directed by volunteers and innovation through an RFC process that is fairly bottom-up, whereas the companies that originated Go and Swift have continued to be more active in stewarding those languages and have pursued the interoperability that align with their language goals. In line with the Rust community’s collaborative ethos and governance model, the typical Rust approach has been to prioritize solutions in external libraries while minimizing changes to the language and standard library[^13]. This approach is evident in Rust’s C++ interoperability story, where most of the work has occurred in external libraries. Rust Project leadership is familiar with this dynamic[^14], and this document is an attempt to explore the problem space in a collaborative, strategic manner.

[^9]: In other words, an ABI. Alternatively, a compiler which understands both languages can translate both to an intermediate representation which then must be correctly combined, but that solution becomes tied to the specific tooling (or theoretically, a standardized IR) rather than being general purpose interoperability between the languages.
[^10]: For example, a C-based pointer has no concept of a valid range, so even though Rust slices and C++ spans are compatible concepts, additional data and correct mapping is necessary to translate between them via a C ABI FFI.
[^11]: According to https://en.wikipedia.org/wiki/System_programming_language#Major_languages
[^12]: “Significant” is subjective, but only Go, Rust, and Swift currently make it into the [TIOBE top 20](https://www.tiobe.com/tiobe-index/).
[^13]: The development of async Rust is a notable example.
[^14]: See Mara Bos’s “[Making Connections](https://www.youtube.com/watch?v=aENHzYAFkeE)” talk and Niko Matsakis’s [Project Goals](https://github.com/rust-lang/rfcs/blob/master/text/3614-project-goals.md).

## The Current Reality

The situation for C++/Rust interoperation today is significantly different from Rust’s inception. While many other tools exist, the three most popular were initially published many years apart on crates.io:

- [bindgen](https://crates.io/crates/bindgen) (call C from Rust): 2015
- [cbindgen](https://crates.io/crates/cbindgen) (call Rust from C): 2017
- [cxx](https://crates.io/crates/cxx) (C++ ↔︎ Rust): 2020

While all three are still actively maintained, the first two are more complete owing to their much longer history and smaller scope. As the most popular crate targeted at C++ interoperation, cxx is decidedly more ambitious and also less complete. The stated goal of cxx is to provide a safe[^15] way to interoperate with C++ with negligible overhead. This is based on the recognition that C++ and Rust share many more high level concepts than either do with C. The goal is to describe this language boundary in Rust and C++, which provides more fidelity than the C ABI used by bindgen and cbindgen. This is a promising direction[^16] and the potential of this approach deserves further exploration. At the same time, there are many limitations and fundamental Rust types which are not yet implemented. Finally, the library is self-described as “restrictive and opinionated” and its goal is not to provide arbitrary interoperability, so much as provide a safer, higher-level abstraction for bridging the narrower conceptual gap between these two languages.

As it stands, interoperability between C++ and Rust is better than it’s ever been, but there are still substantial challenges that serve to limit the feasibility of introducing Rust to existing C++ codebases as well as restricting its use to well-defined boundaries rather than arbitrary interoperation. Finally, it bears mentioning that integrating with build systems is a particular challenge to interoperability, but a strategy for addressing it has yet to be developed. It is likely that beneficial approaches will emerge as the interface-level strategies are refined into tactical work.

[^15]: That is, without requiring cxx consumers to use unsafe Rust code; all C++ code is inherently unsafe, in the sense that it may result in undefined behavior.
[^16]: So much so that it’s been adopted by Chromium and [referenced in Google’s “Comprehensive Rust” guide](https://google.github.io/comprehensive-rust/chromium/interoperability-with-cpp.html)

## Consequences

All use of technology implies trade-offs and the adoption of any new technology involves friction first for technologists to learn its use and second to integrate into systems where interoperation with legacy technologies is required. It is neither feasible nor desirable to replace all C++ code with Rust[^17]. The determination of when rewriting or augmenting C++ systems with Rust code entails a net benefit requires analyses on a per-system basis. Regardless of the specific implementation choices, the purpose of technology is to improve the lives of people. To that end, reducing unnecessary friction and maximizing freedom in the choices of technologists is both in line with the interoperability initiative and the greater good.

[^17]: See “[C++ Must Become Safer](https://www.alilleybrinker.com/blog/cpp-must-become-safer)”

## The Goal(s)

Per the vision stated at the outset, the goal is to make interoperability “accessible and approachable”, but what that looks like differs a great deal depending on the audience. One way to conceptualize the pinnacle of accessibility is to consider the ideal of frictionless interoperability: whether from Rust or C++, *using* code in the other language is no more difficult than if it were implemented in the same language. The fact that each language has fundamentally different abstractions means this isn’t an achievable goal any more than perfect translation between natural languages, but it is still a useful ideal because it can provide a heading along which to proceed and thereby identify goals along the way as well as intervening obstacles.

Some ideals of frictionless interoperability worth pursuing include:
- Minimum toil: the amount of additional user-written code required is no more than the native language
- No added complexity: the details of the interface itself aren’t relevant to code on either side of it
- Maximum safety: interoperating should require the minimal amount of unsafe code
- Maximum correctness: incorrect use of the interoperability facilities themselves should be a compile time error[^18]
- Maximum performance: interoperating should aspire to the [zero overhead principle](https://isocpp.org/wiki/faq/big-picture#zero-overhead-principle)

At this stage, it’s premature to distill these qualities into tactically actionable goals, but viewed as a grand strategy, frictionless interoperation can be pursued through three, parallel strategies:
1. Short-term: improve the existing tools, resolve issues which have been stalled due to lack of ownership, concerted effort or hesitancy to stabilize an implementation
2. Long-term: build the foundations for a richer form of interoperability at the language, compiler, and standard library levels
3. Social interoperability: engage with the C++ community including its users and standards processes to build the bridge from both sides and simultaneously improve both languages

Relevant to all strategies, the initiative will embody several design axioms:
- Build the foundations for a better future while actively improving the present
- Pursue high-quality interoperation from both sides
- Pursue general-purpose interoperability (not tied to a specific toolchain/IR)
- Avoid changes to Rust itself that would undermine its core values
- Only change the language or standard library where external library solutions are insufficient

[^18]: Part of the challenge in achieving this ideal is *defining* correct usage. One aspect of that challenge is already being undertaken in the Foundation's work on the Rust Specification.

## Conclusions and Next Steps

As a problem statement and strategic vision, this document describes the future work of the Rust Foundation’s C++/Rust Interop Initiative. It serves to provide transparency and encourage input and collaboration with the Rust Project as well as the Rust and C++ communities. Most concretely, it serves as a structure to allocate the material financial and staffing resources of the initiative. Ultimately, the direction of C++/Rust interoperability will be determined by the actions of many individuals in both communities and significant work on both short- and long-term efforts is underway. To date, there has been little work to facilitate social cooperation between the two languages, and while better solutions for users of both languages are more likely to emerge from that strategy, all three strategies are independent and mutually reinforcing. Here are the next steps for each strategy:

### Short-term

The greatest benefit which can be realized in the short-term will build on the solutions which are already successful and used in practice today. Distilling this strategy into viable tactics will involve 3 pieces:

1. Identifying issues in libraries, Rust itself or dependencies[^19] which are impeding active interoperability efforts today
2. Prioritizing issues which represent a good value proposition for the material, technical or social resources at the initiative’s disposal
3. Applying the resources through Foundation staffing or materially supporting external technologists

[^19]: For example, a longstanding obstacle to 128-bit integer interoperability recently saw [significant progress](https://blog.rust-lang.org/2024/03/30/i128-layout-update.html) that relied on an LLVM change that took years.

### Long-term

In order to make dramatic improvements in interoperability, incremental improvements are insufficient, so the long-term strategy will seek to practically move towards the ideal of frictionless interoperability. This requires a fundamentally richer interface between languages focused on the shared conceptual space between C++ and Rust with a focus on safety and efficiency. To be useful, this necessitates broad buy-in and cooperation across different Project teams. The pieces of this strategy are:

- Establishing relationships and gathering input and support from key stakeholders in the Project and other relevant experts
- Determine a viable structure to discuss and make high-level decisions about what foundational pieces are required for building a richer interoperability experience and which avenues will not be pursued
- Once actionable foundational pieces are determined, pursue the support of individual teams (perhaps via [Project Goals](https://rust-lang.github.io/rust-project-goals/)) and allocate resources via Foundation Grants

The first piece is well underway and will be ongoing. The second piece requires a top-down, problem-space approach which is somewhat unconventional for the Rust Project and community, but there seems to be support for improving this capability. This represents one of the core challenges and potential benefits of the initiative. The third piece depends somewhat on the success of the first iteration of the Project Goals, but so far appears to be a good fit for supporting new, significant work between the Project and Foundation.

### Social Interoperability

The third strategy is based on the recognition that while Rust was invented with the intention of providing a safer alternative to C++, both languages will continue to exist for the foreseeable future and high-quality interoperability will tend to improve the projects that use both languages. Some amount of competitiveness between languages which target similar use cases is inevitable, but there is more to be gained from cooperation as technological innovation is not a zero-sum game. The pieces of this strategy are:

- Establishing relationships and gathering input and support from key stakeholders in the C++ standards bodies and other relevant experts
- Facilitating introductions and discussion between members of both communities to identify shared values and mutually beneficial strategies
- Determining if any formalized cooperation can be established and work to support it through the resources and membership of the Foundation

Of the three strategies, this is perhaps the most ambitious and least certain. It is also the one which is likely to take the longest to mature, so work has already begun, but none of the other strategies are dependent upon it.

### Next Steps

This problem statement and associated strategies are the first step towards engaging the broader Rust and C++ communities. There is still much to be decided and new ideas and constructive input will be incredibly valuable. If you’d like to provide feedback or suggestions, you can follow discussions at the [t-lang/interop](https://rust-lang.zulipchat.com/#narrow/stream/427678-t-lang.2Finterop) Zulip channel[^20] or email interop@rustfoundation.org. Future updates will be announced in both places as well as the [Rust Foundation blog](https://foundation.rust-lang.org/news/). The initiative will also provide updates to the Rust Foundation board on this work, which includes members of the Rust Project.

[^20]:  As this is part of the rust-lang Zulip instance, the majority of the presence will be people with an interest in the Rust Project, but people with a focus on C++ who are interested in improving C++/Rust interoperability are very welcome!
