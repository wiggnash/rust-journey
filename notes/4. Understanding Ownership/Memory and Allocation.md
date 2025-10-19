
📌 **Related:** [[String Literal and String Type in RUST]]

## String Literals vs String Type

### String Literals

- Contents are **known at compile time**
- Text is **hardcoded directly** into the final executable
- **Fast and efficient**
- ⚠️ **Catch:** These properties only work because string literals are **immutable**

### String Type

- **Mutable** — text can grow at runtime
- Size is **unknown at compile time**
- Must allocate memory on the **heap** to hold its content

---

## Memory Allocation for String Type

Since `String` can grow, it requires dynamic memory allocation:

1. Memory must be **requested from the memory allocator at runtime**
2. When we're done, we need a way to **return this memory** to the allocator

### Example

```rust
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{s}"); // this will print `hello, world!`
```

**Requesting memory** is done using `String::from` — this happens in every programming language.

---

## Memory Deallocation: Different Approaches

### Languages with [[Garbage Collector]]

- Tracks variables automatically
- Cleans up memory that is not used anymore
- No manual intervention needed

### Languages without Garbage Collector

It's **our responsibility** to:

- Identify when memory is no longer being used
- Call code to explicitly free the memory

#### Problems with Manual Memory Management ⚠️

1. **Forget to free** → Memory leak (wasting memory)
2. **Free too early** → Invalid variable
3. **Free twice** → Bug (double-free error)

---

## Rust's Solution: Automatic Memory Management

📌 **Related:** [[Ownership Rules in Rust and Variable Scope]]

In Rust, **memory is automatically returned** once the variable that owns it goes out of scope.

```rust
{
    let s = String::from("hello"); // s is valid from this point forward
    
    // do stuff with s
} // this scope is now over, and s is no longer valid
```

### The `drop` Function

When a variable's scope ends and it's no longer valid, Rust automatically calls a special function called **`drop`**.

- The `String` type author writes code in `drop` to return the memory
- Rust **automatically calls** `drop` at the closing curly bracket `}`

---

> 💡 **Note:** In C++, this pattern of deallocating resources at the end of an item's lifetime is called **Resource Acquisition Is Initialization (RAII)**. The `drop` function in Rust will be familiar if you've used RAII patterns.