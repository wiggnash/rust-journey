**Reference:** [Scalar Types - The Rust Programming Language](https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types)

---

## Overview

**Scalar types** represent a **single value**.

💡 **Analogy:** Similar to primitive data types in JavaScript (number, boolean, etc.)

---

## Types of Scalar Types

Rust has four primary scalar types:

1. **[[Integer Types in Rust]]** - Whole numbers
2. **[[Floating point Types in Rust]]** - Decimal numbers
3. **[[Booleans Types in Rust]]** - True/false values
4. **[[Characters Types in Rust]]** - Single characters

---

## Numeric Operations

Rust supports all basic mathematical operations for numeric types:

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder (modulo)
    let remainder = 43 % 5;
}
```

### 📌 Key Points

- **Addition (`+`)**: Works with integers and floats
- **Subtraction (`-`)**: Works with integers and floats
- **Multiplication (`*`)**: Works with integers and floats
- **Division (`/`)**:
    - Float division returns a float
    - Integer division **truncates** toward zero (e.g., `-5 / 3` = `-1`)
- **Remainder (`%`)**: Returns the remainder of division (modulo operation)

---

## ⚡ Important Notes

- Integer division **truncates** the decimal part
- Use floating-point types when you need decimal precision
- All operands in an operation must be of the same type