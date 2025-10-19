Link to the resource : [Ownership and Functions](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions)

## Core Concept: Move vs Copy

When you pass a value to a function OR assign it to another variable, Rust will either **move** or **copy** it.

### Why This Matters (Coming from JavaScript/Python)

In JavaScript:

```javascript
let name = "Alice";
let anotherName = name;  // Both point to same "Alice" in memory
```

In Rust:

```rust
let name = String::from("Alice");
let another_name = name;  // Ownership MOVES - name is no longer valid!
```

**Key Difference**: Rust prevents the **double-free bug** by ensuring only ONE owner can free memory.

---

## Move vs Copy - The Rule

### Types that are COPIED (Stack-only data)

- Simple types with known, fixed size
- Examples: `i32`, `f64`, `bool`, `char`
- Lives entirely on the **stack**
- Cheap to duplicate
- Both variables remain valid after assignment

```rust
let x = 5;
let y = x;  // x is copied, both x and y are valid
println!("{x}");  // ✓ Works fine
```

### Types that are MOVED (Heap data)

- Complex types with dynamic size
- Example: `String`, `Vec`, custom types without Copy trait
- Has data on the **heap**
- Expensive to deep copy
- Original variable becomes invalid after move

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 is moved, only s2 is valid now
// println!("{s1}");  // ✗ Compile error!
```

---

## Ownership in Functions

### Passing to Functions

**The same move/copy rules apply when passing to functions!**

#### Example: Moving Ownership

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);  // s moves into function
    // println!("{s}");  // ✗ Error! s is no longer valid
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}  // some_string goes out of scope, memory is freed
```

**Think of it as**: `let some_string = s;`

#### Example: Copying Values

```rust
fn main() {
    let x = 5;
    makes_copy(x);  // x is copied (i32 implements Copy trait)
    println!("{x}");  // ✓ x is still valid!
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}  // some_integer goes out of scope, nothing special happens
```

---

## Return Values and Ownership

**Returning values transfers ownership OUT of the function.**

### Example 1: Giving Ownership

```rust
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string  // Ownership moves OUT to caller
}

fn main() {
    let s1 = gives_ownership();  // s1 now owns "yours"
}
```

**Think of it as**: `let s1 = some_string;`

Without this transfer, we'd have a **use-after-free bug** (accessing freed memory).

### Example 2: Taking and Returning Ownership

```rust
fn takes_and_gives_back(a_string: String) -> String {
    a_string  // Return moves ownership back out
}

fn main() {
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // s2 moved in, s3 gets it back
    // println!("{s2}");  // ✗ Error! s2 is no longer valid
}
```

**Ownership flow**:

1. `s2` owns "hello"
2. `s2` → `a_string` (moved into function)
3. `a_string` → `s3` (moved out via return)
4. `s2` is invalid, `s3` owns "hello"

---

## The Problem: Too Much Ceremony

Having to return ownership every time is tedious!

### The Annoying Pattern

```rust
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)  // Have to return String just to keep using it!
}

fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);  // Get both back as tuple
    println!("The length of '{s2}' is {len}.");
}
```

We had to:

- Pass `s1` (lose it)
- Return it back as `s2` (get it back)
- Also return the length we calculated

**This is annoying for something that should be simple!**

---

## The Solution: References (Coming Next)

Rust has a feature called **references** that lets functions use values WITHOUT taking ownership.

This is similar to how JavaScript/Python work - you can pass something to a function and still use it afterward.

---

## Quick Reference

|Action|Move or Copy?|Original Valid?|
|---|---|---|
|`let y = x` where x is `i32`|Copy|✓ Yes|
|`let s2 = s1` where s1 is `String`|Move|✗ No|
|Pass `i32` to function|Copy|✓ Yes|
|Pass `String` to function|Move|✗ No|
|Return `String` from function|Move|Ownership transferred|

---

## Memory Safety Bugs Prevented

1. **Double-free bug**: Two variables trying to free the same memory
2. **Use-after-free bug**: Using memory that's already been freed

Rust's ownership system prevents these at **compile time**!