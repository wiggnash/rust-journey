# Boolean Types in Rust

**Reference:** [The Boolean Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-boolean-type)

---

## Overview

Booleans represent **logical values** and have only two possible states.

---

## Boolean Values

- `true`
- `false`

---

## Type Representation

- **Type keyword:** `bool`
- **Size:** 1 byte (8 bits)

---

## Example

```rust
fn main() {
    let t = true;           // inferred as bool
    let f: bool = false;    // explicit type annotation
}
```

---

## 💡 Key Takeaways

- Only two possible values: `true` or `false`
- Takes up **1 byte** of memory
- Commonly used in conditionals (`if`, `while`, etc.) and logical operations