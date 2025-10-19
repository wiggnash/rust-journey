## The `main` Function

```rust
fn main() {
    println!("Hello, World!");
}
```

### Why is it called `main`?

The `main` function is **special** in Rust:

- It's the **entry point** of every executable Rust program
- Always the **first code that runs** when you execute your program
- Every executable Rust program must have a `main` function

---

## Understanding `println!`

```rust
println!("Hello, World!");
```

### What is `println!`?

- `println!` is a **Rust macro**, not a regular function
- The **`!`** indicates it's a macro
    - With `!` → macro
    - Without `!` → regular function

### What are Macros? (The Basics)

**Key differences between macros and functions:**

1. **Code generation** → Macros generate code to extend Rust's syntax
2. **Different rules** → Macros don't follow the same rules as functions
3. **Flexibility** → Macros can:
    - Accept a **variable number of arguments**
    - Do things that **regular functions cannot**

💡 Macros help us write **flexible code** that adapts to different use cases.

---

## Compiling and Running Rust Programs

### The Compilation Process

```bash
rustc main.rs
```

**What happens:**

1. Rust program (`main.rs`) → **compiler** (`rustc`)
2. Compiler produces a **binary executable file**
3. You run the executable to run your program

### Rust vs Dynamic Languages

|Rust (Compiled)|Dynamic Languages (JS, Python)|
|---|---|
|Compile first, then run|Interpreted at runtime|
|Produces executable|Requires interpreter installed|

### ⚡ Ahead-of-Time Compilation

Rust is an **ahead-of-time compiled language**:

- You **compile the program once**
- Share the **executable** with others
- They can **run it without Rust installed**

**Benefit:** No need for the recipient to have the Rust toolchain—just the compiled binary!

---

## Key Takeaways ✅

- `main` function is the entry point of Rust programs
- `println!` is a macro (note the `!`)
- Macros provide flexibility beyond regular functions
- Rust compiles to native executables
- Executables can run standalone without Rust installed