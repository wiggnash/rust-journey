**Reference:**  [Floating-Point Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#floating-point-types)

---

## Overview

Rust has **two floating-point types** for representing decimal numbers.

---

## Floating Point Types

|Type|Size|Precision|Default|
|---|---|---|---|
|`f32`|32 bits|Single precision|❌|
|`f64`|64 bits|Double precision|✅|

### **`f64`** - Default Type

- **64 bits** in size
- Can hold **more precise** values
- Used by default when Rust infers a floating-point type

### **`f32`**

- **32 bits** in size
- Less precise than `f64`
- Useful when memory efficiency is critical

---

## IEEE-754 Standard

Rust implements floating-point numbers using the **IEEE-754 standard**, which is the industry standard for floating-point arithmetic.

---

## Example

```rust
fn main() {
    let x = 2.0;      // f64 (default)
    let y: f32 = 3.0; // f32 (explicit)
}
```

---

## 💡 Key Takeaways

- **Default:** `f64` for better precision
- **Both types** support standard mathematical operations
- Follow the **IEEE-754 standard** for consistency across platforms
- Choose `f32` only when performance/memory optimization is needed