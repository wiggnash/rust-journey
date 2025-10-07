we will be using rand crate which can generate an random number according to how we want 

## what is a crate ?

Crate is a collection of Rust source code files . There are two types of crates

1. binary crate : this provides an executable
2. library crate : this contains code that is intended to be used in other programs and cannot be executed on its own

To get any crate we need to update the Cargo.toml file . we will add the crate as the dependency

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5"
```

under the dependencies section we can add all the crates that we want to use in our program and cargo understands the semantic versioning : SemVer this is the standard for writing version numbers

Cargo fetches the code from Crates.io this is where all the open source rust projects are present
