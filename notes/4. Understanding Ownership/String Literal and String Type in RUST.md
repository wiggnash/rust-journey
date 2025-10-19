## String Literals

String literals are **fixed and hardcoded** into our program.

### Characteristics

- Size is **fixed** and known at compile time
- Stored in the program binary
- **Immutable** by default

```rust
let s = "Hello!";
```

---

## String Type

When data size is **unknown at compile time**, we use the `String` type, which stores data on the **heap**.

### Key Differences from String Literals

- Stored on the **heap** (not stack)
- Can store text of **unknown size** at compile time
- **Mutable** — value can be changed during runtime

### Example

```rust
let mut s = String::from("Hello");
s.push_str(", World!"); // appends some value to s
println!("{s}");
```

### Understanding the Syntax

- **`String::from()`** — Creates a `String` from a string literal
- **`::`** operator — Indicates that `from` is a function associated with the `String` type
- **`push_str()`** — Method to append text to the string

### Memory Behavior

- **Stack types:** When size is known, data is stored on the stack
    - Comes into scope when declared
    - Popped off when goes out of scope
- **String type:** Data stored on the **heap**
    - Allows for dynamic sizing
    - Enables mutability

---

💡 **Key Takeaway:** `String` type is **mutable** and can grow/shrink at runtime, unlike string literals which are immutable and fixed-size.