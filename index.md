---
layout: default
title: GCC Front-End For Rust
---

This is a full alternative implementation of the Rust language on top of GCC with the goal to become fully upstream with the GNU toolchain.

As this is a front-end project, the compiler will gain full access to all of GCC's internal middle-end optimization passes which are distinct from LLVM. For example, users of this compiler can expect to use the familiar -O2 flags to tune GCC’s optimizer. Going forward, we will be happy to see more LLVM vs GCC graphs in respect to compilation speed, resulting code size and performance. 

The project is still in an early phase with the goal to compile the offical Rust test suite. While there is no borrow-checker included with the compiler yet, we plan to integrate with the [`polonius`](https://github.com/rust-lang/polonius) effort and leverage its borrow-checking algorithms. 

You can find compiler status reports over on: [https://github.com/Rust-GCC/Reporting](https://github.com/Rust-GCC/Reporting), [https://thephilbert.io/](https://thephilbert.io/) as well as here in the next section.

## Status reports

{% assign sorted_posts = site.posts | sort: "date" | reverse %}
<ul>
  {% for post in sorted_posts %}
    <li>
      <a href="{{ post.url }}">{{ post.title }}</a>
    </li>
  {% endfor %}
</ul>

### FAQ

For frequently asked questions please see: [https://github.com/Rust-GCC/gccrs/wiki/Frequently-Asked-Questions](https://github.com/Rust-GCC/gccrs/wiki/Frequently-Asked-Questions)

### Thanks

Thank you to [Open Source Security Inc.](https://www.opensrcsec.com/) and [Embecosm](https://www.embecosm.com/) for sponsoring this project to move forward.

We appreciate all feedback from individuals on github.

### GCC Rust Core Team

* [Philip Herron](https://github.com/philberty/)
* [Arthur Cohen](https://github.com/CohenArthur)
* [Thomas Schwinge](https://github.com/tschwinge)
* [SimplyTheOther](https://github.com/simplytheother)
* [Mark Wielaard](https://gnu.wildebeest.org/blog/mjw/)
* [Marc Poulhiès](https://github.com/dkm)
* [David Faust](https://github.com/dafaust)
* [Wenzhang Yang](https://github.com/thomasyonug)
* [Philipp Krones](https://github.com/flip1995)

### Get Involved

* Github: [https://github.com/Rust-GCC](https://github.com/Rust-GCC)
* Zulip: [https://gcc-rust.zulipchat.com/](https://gcc-rust.zulipchat.com/)
* Twitter: [https://twitter.com/gcc_rust](https://twitter.com/gcc_rust)

### RSS

You can stay updated on our progress with this [RSS feed](/feed.xml).
