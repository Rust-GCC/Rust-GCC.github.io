## GCC Front-End For Rust

This is a full alternative implementaion of the Rust language ontop of GCC which the goal to become fully upstream with the GNU toolchain.

As this is a front-end project, the compiler will gain full access to all of GCCs internal middle-end optimization passes which are distinct from LLVM. For example, users of this compiler can expect to use the familiar -O2 flags to tune GCC’s optimizer. Going forward, we will be happy to see more LLVM vs GCC graphs in respect to compilation speed, resulting code size and performance. 

The project is still in an early phase with the goal to compile the offical rust test suite. There are no immediate plans for a borrow checker as this is not required to compile rust code and is the last pass in the RustC compiler. This can be handled as a sperate project when we get to that point.

You can find compiler status reports over on: [https://github.com/Rust-GCC/Reporting](https://github.com/Rust-GCC/Reporting and [https://thephilbert.io/](https://thephilbert.io/)

### Thanks

Thank you to [Open Source Security Inc.](https://www.opensrcsec.com/) and [Embecosm](https://www.embecosm.com/) for sponsering this project to move forward.

We apreciate all feedback from individuals on github.

### Developers

* [Philip Herron](https://github.com/philberty/)
* [SimplyTheOther](https://github.com/simplytheother)
* [Nala Ginrut](https://github.com/NalaGinrut)

### Get Involved

As this is destined to be upstreamed to GCC we require copyright assignment: [https://gcc.gnu.org/contribute.html](https://gcc.gnu.org/contribute.html). Not all contributions must be code, please try it out and feed us bugs.

* Github: [https://github.com/Rust-GCC](https://github.com/Rust-GCC)
* Zulip: [https://gcc-rust.zulipchat.com/](https://gcc-rust.zulipchat.com/)
* Twitter: [https://twitter.com/gcc_rust](https://twitter.com/gcc_rust)
