## 📚 Resource

[Variables and Mutability - The Rust Book](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability)

---
### Other Concepts
[[Constants in Rust]]
[[Shadowing in Rust]]

---
## 🔒 Immutability by Default

Variables in Rust are **immutable by default**. This means once a value is assigned to a variable, it **cannot be changed**.

### What does "immutable" mean?

Once a value is given to a variable, you cannot reassign or modify that value.

### Example: Immutable Variable Error

```rust
fn main() {
    println!("Hello, world!");
    let x = 5; // assignment 1
    println!("The value of x is : {x}");
    x = 6; // assignment 2 - ERROR!
    println!("The value of x is : {x}");
}
```

**Error Output:**

```bash
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:5:5
  |
3 |     let x = 5;
  |         - first assignment to `x`
4 |     println!("The value of x is : {x}");
5 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
3 |     let mut x = 5;
  |         +++
```

### ✅ Why Immutability Matters

Immutability helps **reduce bugs** caused by accidentally changing variable values during program execution. It makes code safer and easier to reason about.

---

## 🔓 Making Variables Mutable

You can make a variable **mutable** by using the `mut` keyword. This allows the variable's value to be changed after initialization.

### Syntax

```rust
let mut variable_name = value;
```

### Example: Mutable Variable

```rust
fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6; // ✅ This works now!
    println!("The value of x is : {x}");
}
```

**Output:**

```
Hello, world!
The value of x is : 5
The value of x is : 6
```

---

## 💡 When is Mutability Helpful?

Mutability is useful when:

- You need to update a variable's value during computation (e.g., counters, accumulators)
- Working with iterative algorithms where state changes over time
- Building data structures that need to be modified

📌 **Note:** The lecture notes indicate examples and further explanation of mutability benefits will be added here.

---

## Key Takeaways

- **Immutable by default**: Rust variables cannot be changed unless explicitly made mutable
- **Use `mut` keyword**: Add `mut` after `let` to make a variable mutable
- **Safety first**: Immutability prevents accidental changes and reduces bugs
- **Intentional mutability**: When you need to change values, Rust makes you declare it explicitly