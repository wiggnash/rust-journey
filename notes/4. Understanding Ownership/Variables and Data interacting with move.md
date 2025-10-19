## Main Concept

**Multiple variables can interact with the same data in different ways** depending on the data type and how Rust manages memory.

---

## Example 1: Simple Values (Stack Allocation)

```rust
let x = 5;
let y = x;
```

- When values are **simple types** with a **known, fixed size**, they are copied directly
- Both variables (`x` and `y`) are created with the value `5` stored on the **STACK**
- Each variable has its own independent copy

---

## Example 2: Complex Types (Heap Allocation)

```rust
let string_1 = String::from("Hello");
let string_2 = string_1;
```

### How `String` is Stored

The **`String`** type consists of **three parts** stored on the **stack**:

1. **ptr** - pointer to the memory address holding the actual string content (on the heap)
2. **len** - how much memory in bytes is currently being used
3. **capacity** - total amount of memory in bytes allocated from the memory allocator

### What Happens During Assignment

```rust
let string_2 = string_1;
```

- Only the **stack data** (ptr, len, capacity) is copied
- The **heap data** (actual string content) is **NOT copied**

**Why this approach?**  
Copying heap data could be very expensive in terms of runtime performance if the data is large.

---

## 💡 The Double Free Issue

```rust
let string_1 = String::from("Hello");
let string_2 = string_1;
```

### The Problem

- Both `string_1` and `string_2` point to the **same memory location**
- When variables go out of scope, Rust calls the `drop` method to clean up heap memory
- If both variables tried to free the same memory, this would cause a **double free error** (a serious bug)

### How Rust Solves This

```rust
let string_2 = string_1;
```

After this line:

- **`string_1` is no longer valid** - it becomes invalidated
- Only `string_2` can access the data
- When `string_2` goes out of scope, the memory is freed only once

---

## 📌 Shallow Copy vs Deep Copy vs Move

### Shallow Copy

- Copies the **reference** instead of the entire object
- Multiple variables point to the same data

### Deep Copy

- Creates a **completely independent** copy
- Like taking a clone of the data

### ⚡ Move (Rust's Approach)

```rust
let string_1 = String::from("Hello");
let string_2 = string_1;
```

**What actually happens:**

1. Stack data (`String` metadata) is copied from `string_1` to `string_2`
2. `string_1` is **invalidated** immediately after
3. This is called a **Move**, not a copy

> Rust will never automatically create "deep" copies of your data , therefore any automatic copy can be assumed to be inexpensive in terms of runtime performance

**Key takeaway:**  
Instead of saying "copy," we say the value **moved** from `string_1` to `string_2`. This ensures memory safety by preventing double frees.




