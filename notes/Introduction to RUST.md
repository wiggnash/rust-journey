Rust is a programming language designed to build **reliable and efficient and faster** software. Below are some detailed insights into Rust, its features, and its ecosystem.

## Why Rust ?
- **Performance and Reliability:**
    - Rust enables the development of software that is both *fast* and *memory-efficient*.
    - It has **no** **garbage collector** and also that we don't have to handle the memory management on our own which can potentially results in bugs , which contributes to its high performance like how its in C or C++.
	    - The **type system and ownership model** guarantee memory safety and thread safety, eliminating many bugs at compile time.

- **High-Level Ergonomics with Low-Level Control:**
    - Rust balances powerful technical capabilities with a great developer experience.
    - It allows developers to **manage low-level details, such as memory usage**, without the traditional hassle associated with such control.

- **Developer Productivity:**
    - *Rust's compiler* acts as a gatekeeper, refusing to compile code with potential bugs, including concurrency bugs.
    - Developers can focus more on business logic instead of tracking down bugs.

## Who Rust is for ?
### For Team for Developers
```
Low-level code is prone to various bugs, which in most other languages can be caught only through extensive testing and careful code review by experienced developers.
```

- In Rust, **the compiler plays a gatekeeper role** by refusing to compile code with these  bugs, including concurrency bugs. 
- By working alongside the compiler, the team can spend their time focusing on the program’s logic rather than chasing down bugs.

### Tools RUST is giving us to use
1. **Cargo** : Dependency manager and Build Tool => Adding , compiling and managing dependencies easy and consistent across the ecosystem
2. **Rustfmt** : Formatting tool for rust
3. **rust-analyzer** : Used in IDE's used for code completion and inline error messages

## Setting Up and Developing in Rust

- **Installing Rust:**
    - **rustup** is a command-line tool for managing Rust versions and associated tools. It simplifies the installation of Rust and its components.
    - For Windows users, it is recommended to use Windows Subsystem for Linux (WSL); this allows for consistent command usage as on Linux distributions. For installation details, refer to the [Rustup Documentation](https://rustup.rs/).
- **Updating Rust:** If you have already installed rust in your system
    ```bash
    $ rustup update
    ```
## Components which are installed with rustup : Just for reference
- `rustc` — The Rust compiler and [Rustdoc](https://doc.rust-lang.org/rustdoc/).
	- This is the heart and soul of rust
	- Complies the rust source code into the binary executable file , this will be executed in any of the OS  
- `cargo` — [Cargo](https://doc.rust-lang.org/cargo/) is a package manager and build tool.
	- Used to install external dependensies or third part component into our project
	- similar to NPM which is there in javascript
- `rustfmt` — [Rustfmt](https://github.com/rust-lang/rustfmt) is a tool for automatically formatting code.
- `rust-std` — This is the Rust [standard library](https://doc.rust-lang.org/std/). There is a separate `rust-std` component for each target that `rustc` supports, such as `rust-std-x86_64-pc-windows-msvc`. See the [Cross-compilation](https://rust-lang.github.io/rustup/cross-compilation.html) chapter for more detail.
- `rust-docs` — This is a local copy of the [Rust documentation](https://doc.rust-lang.org/). Use the `rustup doc` command to open the documentation in a web browser. Run `rustup doc --help` for more options.
- `rust-analyzer` — [rust-analyzer](https://rust-analyzer.github.io/) is a language server that provides support for editors and IDEs.
- `clippy` — [Clippy](https://github.com/rust-lang/rust-clippy) is a lint tool that provides extra checks for common mistakes and stylistic choices.
- `miri` — [Miri](https://github.com/rust-lang/miri/) is an experimental Rust interpreter, which can be used for checking for undefined-behavior.
- `rust-src` — This is a local copy of the source code of the Rust standard library. This can be used by some tools, such as [rust-analyzer](https://rust-analyzer.github.io/), to provide auto-completion for functions within the standard library; [Miri](https://github.com/rust-lang/miri/) which is a Rust interpreter; and Cargo’s experimental [build-std](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-std) feature, which allows you to rebuild the standard library locally.
- `rust-mingw` — This contains a linker and platform libraries for building on the `x86_64-pc-windows-gnu` platform.
- `llvm-tools` — This component contains a collection of [LLVM](https://llvm.org/) tools. Note that this component has not been stabilized and may change in the future and is provided as-is. See [#85658](https://github.com/rust-lang/rust/issues/85658).
- `rustc-dev` — This component contains the compiler as a library. Most users will not need this; it is only needed for development _of_ tools that link to the compiler, such as making modifications to [Clippy](https://github.com/rust-lang/rust-clippy).