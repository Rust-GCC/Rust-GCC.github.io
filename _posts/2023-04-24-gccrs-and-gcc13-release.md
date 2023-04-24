---
layout: post
title: "GCC 13 and the state of gccrs"
author: Philip Herron and Arthur Cohen
tags:
    - meta
---

The first official GCC 13 release is just around the corner, and you may be a bit surprised to find out that gccrs will not be included in it. The aim of this blogpost is to go into a little bit more detail as to why this was done and how you can still try out our compiler if you'd like to.

Back in 2022, we started investigating the process of merging our code upstream with GCC.
Quickly after that, our frontend was accepted for the merge, which was a great moment for us -
it demonstrated people's faith in the `gccrs` community and in the project's goal of having an
alternative GCC toolchain implementation of the Rust Programming Language. Given that the compiler is still not ready, we decided to go through that process with some limitations.

The expectation we had was that by making it clear the compiler was still not ready for real
Rust code, we could get valuable feedback from early users as well as more engagement from
two different communities - the Rust one as well as the GCC one. We would continue to push
as hard as possible right up until the release's deadline to get as much working as we could,
but now that the GCC 13.1 release is upon us, we are still not there in terms of completeness.

What this means is that when using `gccrs`, you are still not able to do "easy Rust things" - examples like "Hello World!", or using the standard library in general, will not work. Similarly, even more complex concepts which are essential to the Rust Programming Language like borrow-checking are not implemented yet, and `gccrs` will not be considered complete without such features. We
believe that this would cause a lot of confusion for unsuspecting users, who might see `gccrs`
as part of an official GCC release and expect a working compiler.

## Why are we not "ready"

Saying that Rust is not an easy language to implement would be an understatement. And while we believe
that Rust being so strict is one of the great strengths of the language, as it restricts the potential for
user errors, nullifies the possibility of undefined behavior in idiomatic code,
and protects the user and environment against a lot of the vulnerabilities found in C and C++ programs, it does come at a cost - *Rust is designed in such a way that
nothing will work, unless everything works.*

But what does this mean?

### The Rust standard library

The standard library can be seen as a few crates depending on each other, with two of the main components being
`core` and `alloc`. The `alloc` crate provides abstractions around dynamic memory allocations in Rust, such as
the `String` or `Vec` type. `core` on the other hand, provides functionality that is much more intrinsic to the
language and which does not require dynamic memory - this ranges from crucial types such as `Option` and `Result`, to
less-known functionality including creating slices, or the implementation of numeric
operations for primitive number types using the `Add`, `Sub`, `Mul` and `Div` traits.

The `core` crate also *declares* abstractions which are implemented directly within the Rust compiler: compiler intrinsics,
builtin macros, lang items... and there is an abundance of these abstractions!

As it stands, `gccrs` cannot yet compile the version of the `core` crate that we are targeting: for example, we are still
not able to compile `for` loops, which depend on some desugaring to pattern matching, as well as iterators, a very complex
lang item and trait all defined within libcore.

[Philip](https://github.com/philberty) is leading the effort here, which heavily affects our
type system and second intermediate representation within the compiler, and will continue
pushing forward.

Furthermore, despite us wanting to target the Rust 1.49 version of the `core` library, we also need to implement features
that have only been stabilized in later versions of Rust, or might still be unstable to this day, as the Rust standard library is one of set of crates making use of new features and trying them out. You can find an
approximate list of the features we will need to handle in order to compiler the `core` crate [here](https://github.com/Rust-GCC/gccrs/issues/1579).

The clever part about this is that by compiling the `core` crate without any modification,
we avoid introducing inconsistencies within the implementation,  as it implements such core
language behaviour. Compounding the difficulty here, Rust 1.49 does not really have a
`#![no_core]` testsuite, so figuring out the proper behaviour requires a lot of investigation
on our end.

Another snippet of code you might expect `gccrs` to handle is the following:

```rust
fn main() {
    println!("Hello, world!");
}
```

but we currently cannot! If you know a little bit about Rust, you'll know that `println!` is a
macro invocation. Macros are complicated, but are also extremely powerful. We have worked on
them for a while now, and some of the more complex features are still not properly implemented.

If you've used `println` in the past, you'll know that it allows for very powerful format strings:

```rust
println!("Hello {name}");
println!("Debugging {something:?} and {:#?}", this_as_well);
println!("printing numbers: {:04}, {:#x}", leading_zeros, hexa_value)
println!("printing dashes ! {:-<5}", "some string");
```

([playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=be823ac3c0e32bc4801ca26121b439cf))

The handling of these format strings is done *within* the compiler, using the builtin macros `format_args!` and `format_args_nl!`.
These macros are extremely complex, as well as the format specifiers they offer to users. Once these specifiers are parsed properly,
we need to think about creating calls to the corresponding `Debug`, `Display` or other-formatting-traits' `fmt` functions. The output resulting of these calls then needs to be interspersed in the original format string, removing the curly brackets or other parts of the specifiers.

While all of this seems quite easy to understand and use, the Rust compiler has to do a lot of work to handle these macro invocations!

As a side note, we also recently started work on procedural macros, with [Pierre-Emmanuel Patry](https://github.com/P-E-P)
leading this effort. Some builtin procedural macros are required to build the `core` crate, such as
builtin derive macros (`#[derive(Clone, Copy)]`) which are quite familiar to Rust programmers. You'll also often see
Rust code use `#[derive(Debug)]`, which ties into our upcoming work on format strings mentioned above.

### How far away are we?

While all of this appears like a lot of work, we are confident in our progress and hope to get closer and closer to getting the `core` crate working
in the next few months. There is also a lot of important work remaining in order to produce a valid Rust compiler, which is why we will spend
the coming months focusing on the `core` crate as well as a borrow-checker implementation, and the development of the necessary tooling to allow
us to try and pass the Rust 1.49 testsuite.

We aim to distribute the Rust 1.49 version of the standard library with our compiler in the next major GCC release, GCC 14, and hope
to backport enough changes to the GCC 13 branch to get the `core` crate working in time for the GCC 13.2 release. This will enable users to easily start experimenting with the
compiler for `#![no_std]` Rust programs and, hopefully, some embedded targets.

In the meantime, if you are still interested in trying out the compiler, and helping us iron out the kinks, there are multiple possibilities:

1. You can compile GCC from source, either from our [GitHub repository](https://github.com/rust-gcc/gccrs) or from the [upstream GCC repository](https://gcc.gnu.org/git.html).
2. Use [Compiler Explorer](https://rust.godbolt.org/)
3. Use our [Docker image](https://hub.docker.com/r/philberty/gccrs). This container ships with a build of `gccrs` as well as `cargo-gccrs`.
4. Finally, we also plan to produce regular binary releases soon - this will allow you to install a working, but unstable, version of `gccrs`, and to use it with `cargo-gccrs` or by itself. These binaries will be available on the [Embecosm website](https://www.embecosm.com/resources/tool-chain-downloads/) shortly, around the time GCC 13.1 releases.

As usual, should you run into any issues getting `gccrs` to compile, feel free to use one of the following links to get in touch:

* [Our Zulip Server](https://gcc-rust.zulipchat.com/)
* [The `gccrs` mailing list](https://gcc.gnu.org/mailman/listinfo/gcc-rust)
* Our IRC channel, on irc.oftc.net in `#gccrust`

### Getting involved

Compilers sadly do not grow on trees, and if you are interested in the project please reach out to us.

- Want to get involved in our core team?
- Able to code?
- Have $ to spend on developer time?
- Able to contibute testing?

Building a new front-end is difficult work and we are always seeking sponsorship opportunities. Please reach out to us, either via email (<herron.philip@googlemail.com> and <arthur.cohen@embecosm.com>), our Zulip group or on GitHub if you wish to get involved at any level.

## Moving foward

Although we did not make it to the finish line for the first GCC 13 release, we are happy with the progress we have made so far and are looking forward to the future.

We have already completed the initial merge with GCC upstream, which took a huge amount of time and will not need to be repeated. This will allow us to focus even *more*
on the compiler's development for GCC 14, and will enable us to push patches upstream easily. This time will be used to produce a higher quality compiler, and to hopefully get close to a real
Rust compiler. This has also increased visibility for the project, with many articles being written about our various pushes upstream.

We will keep on working hard towards making a production quality Rust compiler front-end
for GCC, and will stick to our existing principles: we want to respect the Rust programming
language, and not to harm it in any way, shape or form. The goal of `gccrs` is not to
circumvent the various processes around the Rust language, nor is it an escape hatch for what
should be invalid Rust code. We are using `rustc` as a guide, and will keep on treating every
difference between the two compilers as a bug.

We'd like to thank each of you reading this blogpost for your continued interest in the project.
We are incredibly thankful to the people supporting us, to those that chose to spend some of
their time contributing with us, as well as to those that are simply watching us from the
sidelines and keeping an eye on the project.

Above all, we would like to thank Open Source Security, inc, and Embecosm, who have continued
to believe in our mission and have given us their full support for the past few years and will continue to do so.

See you all very soon!