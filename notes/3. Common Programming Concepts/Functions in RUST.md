**Reference:** [Functions - The Rust Programming Language](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions)

---

## Overview

- **`main`** function is the **entry point** of most Rust programs
- **`fn`** keyword is used to declare new functions
- Function and variable names use **`snake_case`** convention (lowercase with underscores)

---

## Function Declaration

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

### 📌 Function Ordering

- Functions can be defined **before or after** they are called
- Order **does not matter**
- The called function must be **in scope** of the calling function

---

## Parameters

**Parameters** are special variables that are part of a function's signature.

### Rules

- Functions can have **multiple parameters**
- **Type annotations are required** for all parameters
- When calling, provide concrete values called **arguments**

### Example

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

---

## Statements and Expressions

Rust is an **expression-based language**. Function bodies consist of statements and optionally end with an expression.

### Statements

- **Instructions** that perform an action
- **Do not return a value**

```rust
fn main() {
    let y = 6;  // This is a statement
}
```

### Expressions

- **Evaluate to a resultant value**
- **Do not include semicolons**
- Adding a semicolon turns an expression into a statement

**Examples of expressions:**

- Calling a function
- Calling a macro
- A new scope block `{}`
- Math operations: `5 + 6`

### Expression Example

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1  // ← Expression (no semicolon)
    };
    println!("The value of y is: {y}");  // Prints: 4
}
```

⚠️ **Critical:** `x + 1` has **no semicolon** - it's an expression that returns a value.

---

## Functions with Return Values

Functions can return values to their callers.

### Syntax

- Declare the return type after an arrow (`->`)
- The return value is the final **expression** in the function body
- **No semicolon** on the return expression

### Example

```rust
fn five() -> i32 {
    5  // ← Expression, returns 5
}

fn main() {
    let x = five();
    println!("The value of x is: {x}");  // Prints: 5
}
```

### ❌ Common Mistake: Adding Semicolon

```rust
fn five() -> i32 {
    5;  // ← Statement! Does not return a value
}
```

**Error:**

```bash
error[E0308]: mismatched types
  --> src/main.rs:23:14
   |
23 | fn five() -> i32 {
   |    ----      ^^^ expected `i32`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
24 |     5;
   |      - help: remove this semicolon to return this value
```

**Why?** Adding a semicolon converts the expression into a statement, which returns `()` (unit type/nothing) instead of `i32`.

---

## 🔑 Key Takeaways

|Concept|Description|Semicolon?|
|---|---|---|
|**Statement**|Performs action, no return value|✅ Yes|
|**Expression**|Evaluates to a value|❌ No|

### Expression vs Statement

```rust
fn main() {
    let x = 5;           // Statement
    let y = {
        let a = 3;       // Statement
        a + 1            // Expression (returns 4)
    };
    println!("{y}");     // Prints: 4
}
```

---

## 💡 Return Value Rules

1. Declare return type with `-> Type`
2. Return value is the **final expression** (no semicolon)
3. Can use explicit `return` keyword for early returns
4. **Semicolon = statement = no return value**

```rust
fn add_one(x: i32) -> i32 {
    x + 1  // ✅ Returns x + 1
}

fn add_one_wrong(x: i32) -> i32 {
    x + 1;  // ❌ Returns nothing, causes error
}
```