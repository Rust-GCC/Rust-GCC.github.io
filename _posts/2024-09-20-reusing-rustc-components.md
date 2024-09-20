---
layout: post
title: "(Re)Using rustc components in gccrs"
author: Philip Herron, Pierre-Emmanuel Patry and Arthur Cohen
tags:
    - meta
    - compiler
    - rustc
    - borrow-checker
---

1. Why
  1. To speed up development
  2. To make sure we are doing exactly the same thing as rustc in those cases
2. What components do we reuse
3. What do we plan on reusing?
  1. Trait solver eventually? as it's crucial and really complex
  2. Auto transmute work by Jack Wrenn -> ping on Zulip to get the name
  3. Why not reuse even more???
    - internal representations like our ASTs are different and ever-changing
    - why bother making another compiler if we are going to reuse everything? :eyes:
4. What does it mean to be the same as rustc for those components?
  1. borrow-checker: Have the exact same errors instead of subtle differences
  2. format-args-parser: Have the exact same internal representation to match with std's format module perfectly
5. How do we use them at the moment?
  1. We build them with cargo/rustc
6. How do we plan on using/compiling them?
  1. We first build gccrs without them
  2. We then use gccrs to build them and link to ourselves
7. Bootstrap diagram

