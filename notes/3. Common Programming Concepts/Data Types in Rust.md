**Reference:** [Data Types - The Rust Programming Language](https://doc.rust-lang.org/book/ch03-02-data-types.html#data-types)

---

## Overview

- Every value in Rust has a specific **data type**
- The data type tells Rust how to work with that data
- Rust is a **statically typed language**: all variable types must be known at compile time

---

## Type Inference vs Explicit Types

Rust can **infer types** based on:

- The value assigned to a variable
- How the variable is used

However, **explicit type annotations** are required when:

- Multiple types are possible
- Converting from one type to another (type parsing/casting)

### ✅ Best Practice

Always specify types when converting between data types to avoid ambiguity.

---

## Example: Type Annotation Requirement

When parsing a string into a number, you **must** specify the target type:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
println!("Parsed Number {guess}");
```

### ❌ Without Type Annotation

If you omit the type, the compiler cannot infer which numeric type to use:

```rust
let guess = "42".parse().expect("Not a number!");
```

**Error:**

```bash
error[E0284]: type annotations needed
  --> src/main.rs:34:9
   |
34 |     let guess = "42".parse().expect("Not a number!");
   |         ^^^^^        ----- type must be known at this point
   |
   = note: cannot satisfy `<_ as FromStr>::Err == _`
help: consider giving `guess` an explicit type
   |
34 |     let guess: /* Type */ = "42".parse().expect("Not a number!");
   |              ++++++++++++
```

**Why?** The `parse()` method can convert to many types (u32, i64, f64, etc.), so Rust needs you to specify which one.

---

## Categories of Data Types in Rust

Rust has two main type categories:

1. **[[Scalar types in RUST]]** - Represent a single value
2. **[[Compound types in RUST]]** - Group multiple values together

---

## 💡 Key Takeaways

- Rust's static typing catches errors at compile time
- Type inference reduces boilerplate in most cases
- Explicit annotations are necessary when ambiguity exists
- Always annotate when parsing or converting types