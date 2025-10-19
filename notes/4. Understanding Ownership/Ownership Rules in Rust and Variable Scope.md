## Ownership Rules

Rust's ownership system is governed by **three fundamental rules**:

1. **Each value in Rust has an owner**
2. **There can only be one owner at a time**
3. **When the owner goes out of scope, the value will be dropped**

---

## Variable Scope

### What is Scope?

A **scope** is the range within a program for which an item is valid — it's like the **visibility of variables** in a program.

### Scope Example

```rust
{                      // s is not valid here, since it's not yet declared
    let s = "hello";   // s is valid from this point forward
    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

### Key Points

- ✅ When a variable is **declared**, it comes into scope and becomes valid
- ✅ It remains **valid** until it goes out of scope
- ❌ In the example above, when the closing bracket `}` is reached, `s` goes out of scope and is no longer valid

---

📌 **Remember:** The compiler automatically cleans up memory when variables go out of scope — this is ownership in action!