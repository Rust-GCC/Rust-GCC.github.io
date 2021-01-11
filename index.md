## GCC Front-End For Rust

This is a full alternative implementaion of the Rust language ontop of GCC which the goal to become fully upstream with the GNU toolchain.

As this is a front-end project, the compiler will gain full access to all of GCCs internal middle-end optimization passes which are distinct from LLVM. For example, users of this compiler can expect to use the familiar -O2 flags to tune GCC’s optimizer. Going forward, we will be happy to see more LLVM vs GCC graphs in respect to compilation speed, resulting code size and performance. 

You can find compiler status reports over on: https://github.com/Rust-GCC/Reporting. The project is still in an early phase with the goal to compile the offical rust test suite. There are no immediate plans for a borrow checker as this is not required to compile rust code and is the last pass in the RustC compiler. This can be handled as a sperate project when we get to that point.

### Developers

* [Philip Herron](https://github.com/philberty/) https://thephilbert.io/
* [SimplyTheOther](https://github.com/simplytheother)
* [Nala Ginrut](https://github.com/NalaGinrut)  https://nalaginrut.com/index

### Get Involved

As this is destined to be upstreamed to GCC we require copyright assignment: https://gcc.gnu.org/contribute.html. Not all contributions must be code, please try it out and feed us bugs.

Github: https://github.com/Rust-GCC
Zulip: https://gcc-rust.zulipchat.com/
Twitter: https://twitter.com/gcc_rust
