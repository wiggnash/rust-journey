**Reference:** [Compound Types - The Rust Programming Language](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types)

---

## Overview

**Compound types** group multiple values into a single type.

---

## 2 Primitive Compound Types in Rust

1. **Tuples** - Group values of different types
2. **Arrays** - Group values of the same type

---

## Tuples

### Characteristics

- Group **multiple values** with **variety of types** into one compound type
- **Fixed length** - cannot grow or shrink after creation
- Each position can have a different type

### Example

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

### Accessing Tuple Values: Destructuring

You can use **pattern matching** to extract tuple values:

```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}
```

📌 **This pattern is called tuple _destructuring_**

### Accessing by Index

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}
```

---

## Arrays

### Characteristics

- Store a **collection of values** of the **same type**
- **Fixed length** - size cannot change
- Data stored on the **stack** (not heap)
- Use when you know the exact number of elements that won't change

### When to Use Arrays

✅ **Use arrays when:**

- You need data on the **stack** (faster access)
- You have a **fixed number** of elements
- The size will **not change**

### Syntax

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

let a: [i32; 5] = [1, 2, 3, 4, 5];  // type and length annotation

let a = [3; 5];  // [initial_value; length] → [3, 3, 3, 3, 3]
```

---

## Vector Type (Alternative to Arrays)

### Characteristics

- **Flexible** size - can grow or shrink
- Contents stored on the **heap**
- Similar to arrays but dynamic

### Example

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];           // array (stack, fixed size)
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];  // vector (heap, flexible)
}
```

💡 **Use vectors when the collection size needs to change during runtime**

---

## Accessing Array Elements

Arrays are a **single chunk of memory** with a known, fixed size allocated on the **stack**.

### Syntax

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];   // 1
    let second = a[1];  // 2
}
```

### ⚠️ Invalid Index Access

**What happens when accessing an invalid index?**

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];  // valid indices: 0-4
    let element = a[5];        // ❌ index 5 is out of bounds!
}
```

**Result:** Rust will **panic** and the program will exit immediately.

### ✅ Safety Advantage

- Rust **prevents invalid memory access**
- The program **stops** instead of allowing access to invalid memory
- This protects against **undefined behavior** and **security vulnerabilities**
- Unlike C/C++, Rust catches these errors at runtime rather than silently corrupting memory

---

## 🔑 Key Takeaways

|Feature|Tuples|Arrays|
|---|---|---|
|**Types**|Different types allowed|Same type only|
|**Size**|Fixed|Fixed|
|**Access**|Destructuring or `.index`|`[index]` notation|
|**Memory**|Stack|Stack|

|Feature|Arrays|Vectors|
|---|---|---|
|**Size**|Fixed|Dynamic|
|**Memory**|Stack|Heap|
|**Use Case**|Known, unchanging size|Flexible, changing size|

---

## 💡 Memory Safety

Rust's array bounds checking at runtime ensures:

- No buffer overflow vulnerabilities
- No accessing freed memory
- Program crashes safely instead of corrupting data