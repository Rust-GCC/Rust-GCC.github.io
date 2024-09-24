---
layout: post
title: "(Re)Using rustc components in gccrs"
author: Philip Herron, Pierre-Emmanuel Patry and Arthur Cohen
tags:
    - meta
    - compiler
    - rustc
    - borrow-checker
    - format-args
    - trait-solver
---

In order to speed up development, as well as make sure `gccrs` exposes the exact same behavior as `rustc` in critical compiler passes, we decided last year to start reusing components and crates from the official Rust compiler where possible. These components range from external crates used for certain `nightly` options to internal `rustc` workspace packages. As expected, these components are written in Rust - which poses an interesting challenge for `gccrs`, a compiler written in C++, to be able to use them. The goal of this blogpost is to explore some of these components, explain why we are reusing them, and talk about how they will be integrated into the compiler once it is more complete.

## Which components?

The first `rustc` component that was added to `gccrs` was [`rustc_parse_format`](https://github.com/rust-lang/rust/tree/c22a4215a0f6fb676d3774d3763d9da1462414f5/compiler/rustc_parse_format), which we integrated at the beginning of the year in 2024. The role of this crate is to parse ["Rust format strings"](https://doc.rust-lang.org/std/fmt/) - which correspond to the various string templates used in string formatting and inline assembly. The crate should parse the template string, as well as each of the "pieces" used in the template string. For example, when looking at the following code:

```rust
println!("Hello {ferris}!")
```

the parser will emit three "pieces" - two for the static strings `"Hello "` and `"!"`, and one for the named argument `ferris`. Slightly simplified, the resulting vector will look like this:

```rust
vec![
    Piece::Static("Hello "),
    Piece::ArgumentNamed(ferris, Trait::Display),
    Piece::Static("!"),
]
```

Once we get these pieces back into the compiler, we can generate Rust code to call into `core::fmt` functions, so that at runtime your code will correctly call into the proper formatting traits like `Display`, `Debug` and so on. For the version of the language we are targeting, that generated code looks something like this:

```rust
io::_print(
    fmt::Arguments::new_v1(
        &["Hello, ", "!\n"],
        &[fmt::ArgumentV1::new(&ferris, fmt::Display::fmt)]
    )
);
```

If you're thinking "Arthur, why didn't you just rewrite the parser, that sounds easy" well the answer is I am absolutely ass at writing parsers in C++. I need [`nom`](https://crates.io/crates/nom) or I can't function.

Regardless, `rustc_parse_format` is a tiny `rustc` component, which in truth is not terribly difficult to implement. But tweaking the build system to allow the reuse of that component, as well as writing the various Rust <-> C++ interfaces needed for interacting with the `crate`, helped pave the way for the most crucial of the components we are currently reusing - `polonius`, the next generation Rust borrow-checker, which you can read more about [here](https://rust-lang.github.io/polonius/). It can already be used on `rustc` with the nightly `-Z polonius` option. It is currently being rewritten, and not stable yet, but will eventually allow a couple more constructs which are rejected by the current borrow-checking algorithm while still being just as correct.

We plan on adding more components to `rustc` in the future where _NOTE: Reword "it makes sense"_. For example, there are currently efforts towards making a new trait solver for `rustc` - if separate enough from the compiler, it could be integrated and used by `gccrs` to perform trait-solving on our internal representation. Similarly, [Jack Wrenn](https://github.com/jswrenn) gave a fantastic talk at RustConf 2024 detailling checked-transmutes _NOTE: Add links to proposals, blogpost, etc_. Talking with Jack after his talk revealed that the algorithm powering the project is quite separate from the compiler. It uses its own graph representation, which is built from `rustc`'s internal representation, but could also be built using `gccrs`'!

While we do want to encourage sharing between the two projects, it is not possible to reuse any component we want from `rustc` - The two compilers' internal representations are extremely different, and converting back-and-forth from one to the other would be extremely costly. A simple but really annoying example of this lies in our AST structure, namely for representing the "block" expressions used in Rust:

```rust
let result = {
    let x = heavy_computation();
    let y = complex_math();

    x + y
}
```

In `rustc`, the structure used looks like this [(taken from rustc's github)](https://github.com/rust-lang/rust/blob/4cadeda932d5c261a9a0b1bbd25c4486e4e0a4c6/compiler/rustc_ast/src/ast.rs#L540):

```rust
pub struct Block {
    pub stmts: ThinVec<Stmt>,
    pub id: NodeId,
    ...
}
```

So for the code above, we'd have the following:

```rust
Block {
    stmts: [ LetStmt(...), LetStmt(...), ArithmeticOperation(...) ],
    id: ...
}
```

Whereas in `gccrs`, we use the following class:

```cpp
class BlockExpr : public ExprWithBlock
{
    std::vector<std::unique_ptr<Stmt>> statements;
    std::unique_ptr<Expr> expr;
    ...
};
```

so the previous code snippet would be represented as this:

```rust
BlockExpr {
    statements: [ LetStmt(...), LetStmt(...) ],
    expr: ArithmeticOperation(...)
}
```

In `rustc`, expressions can be statements - hence, the last expression of a block can simply be represented as a statement. In `gccrs`, this isn't the case, so we have to represent the tail expression as a separate member. Obviously, this has repercussions on how certain algorithms in both compilers should treat block expressions, and thus cannot be used interchangeably.

## Why is it important to be the exact same as rustc?

Borrow-checking is an extremely complex subject, and a core part of the Rust programming language. It is important that `gccrs` gets it right, and it is important for us not to introduce subtle differences with `rustc` for such a crucial error pass. Instead of rolling out our own borrow-checking algorithm, reusing one which will be used by `rustc` allows us to at least reduce the amount of differences we will introduce. Of course, we still need to be extremely careful when creating the information used by `polonius`, and to test the output extensively. But we already *know* that `polonius` itself has been tested extensively within `rustc`, and will continue to be tested once it gets integrated into the official compiler. This similar reasoning can be applied to the future trait-solver. The same reasoning can be applied to trait-solving, and other user-facing, complex compiler passes. While we currently have a trait-solver, and can use it to typecheck real-world Rust code, it will not be close to the work done by a dedicated team over multiple years to achieve *next generation trait-solving*.

For string templating, making sure that we parse template string properly is important, as the compiler needs to emit code calling into Rust's standard library. By using the exact same parser crate, we can be sure that we generate the same runtime calls to the standard library's formatting module, thus helping us ensure users will have the same behavior with `rustc` and `gccrs` when using string formatting. This also helps ensure that inline assembly nodes are constructed properly, as `rustc` uses the same crate to parse them.

## How do we use them?

The components are written in Rust. We are writing a Rust compiler. Seems like we could just... reuse our project to compile these components? And yes, we can! But not just yet. At the moment, `gccrs` is still a work in progress compiler, and cannot compile a lot of real-world Rust code out there - including `rustc_parse_format` and `polonius-engine`. For this reason, we currently rely on `cargo` and `rustc` to compile these crates for us, and we then link them to our compiler's executable at the end of the compilation process. Once `gccrs` is more complete, we plan to integrate these crates to our compiler using the following bootstrapping process:

<div style="text-align:center;">
    <img src="/images/reusing-rustc-components-1.svg" width="60%">
</div>

First, we use a C++ compiler to build `gccrs`, disabling the borrow-checking pass of the compiler. It is important to ensure borrow-checking cannot be disabled at runtime using a command line flag, as this could have a negative impact on the Rust ecosystem - however, building `gccrs` without borrow-checking to use it as an intermediate compiler in our bootstrapping process is okay. Similarly, secret powers can be unlocked from `rustc` if you define certain environment variables, which among other things allow a stable compiler to compile the Rust standard library - which relies heavily on nightly Rust features.

Since this intermediate bootstrapping compiler will also be built without any of the Rust components we plan on using, it will be quite limited. For example, its string formatting capabilities will be nil, and it will not be able to be used for regular Rust display operations - its sole purpose will be to build the `polonius-engine` crate. Other Rust components we may depend on will either be "mocked", or replaced by an alternative, simpler and less complete re-implementation in C++.

Once this intermediate compiler is built, we use it to compile `polonius`, as well as the rest of the Rust components we use. We can then use these crates and link them to `gccrs`, giving us a proper compiler with borrow-checking enabled, and string formatting capabilities. To make sure the crates we have compiled are valid, we need to compile them once again with our newly-built complete compiler, or exit the bootstrapping process if this fails.

And voila! We now have a Rust compiler, with Rust components inside it. [Dogfed](https://en.wikipedia.org/wiki/Eating_your_own_dog_food).
