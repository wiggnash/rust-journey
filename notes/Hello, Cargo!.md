what is cargo ?
Rust's build system
package manager
we can use this tool to manage projects : building the code and handling dependencies
therefore we can add dependencies to our project very easily using cargo tool

helps to organise our project , that there is place for everything

create a new project using cargo
```bash
cargo new hello_cargo
```

cargo.toml file
TOML : Tom's Obvious, Minimal Language format

in rust packages of code are reffereed to as crates

## Building and Running a cargo project

Cargo build

```bash
cargo build
```

this creates an executable file inside a specific directory
the default build is debug build

creates Cargo.lock file : this will remember the versions of the dependencies of our project , this is managed by cargo and we do not need to update anything in this file

Cargo run
```bash
cargo run
```

this will compile and run the executable in one command

Cargo check

```bash
cargo check
```

we can check our code to make sure it compiles but it does not produce any executable file
always recommended to use this command to quickly check any issues or errors and it is faster than cargo build

once we are sure about everything then we can cargo build