# Rust References and Borrowing

## The Ownership Problem

### Initial Issue with Ownership Transfer

```rust
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
```

**Problem:** We need to return the `String` to the calling function so we can still use it after passing it to the function.

**Solution:** Use references to refer to values without taking ownership.

---

## What is a Reference?

A **reference** is like a pointer that holds an address. Key properties:

- Contains an address that points to data
- The data is **owned by another variable**
- The reference points to a **valid value** for its entire lifetime
- Represented by the `&` symbol

💡 **Core concept:** References allow you to refer to a value without taking ownership.

📌 Related: [[Pointer and References]]

---

## Using References (Borrowing)

### Fixed Code with Reference

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

**How it works:**

- `&s1` means "address of s1" (creating a reference)
- No transfer of ownership occurs
- When the reference goes out of scope, `s1` still points to the heap value

```rust
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // s goes out of scope here, but doesn't drop the data
```

### Borrowing Analogy

Creating a reference is called **borrowing** — like borrowing a car from a friend. You use it and return it, but you don't own it.

---

## Immutability of References

**Can we change what we're borrowing?**

❌ **No** — references are **immutable by default**.

### This Will Not Compile:

```rust
fn main() {
    let s = String::from("Hello");
    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", World"); // ❌ Error: cannot modify borrowed content
}
```

---

## Mutable References

To modify borrowed data, use a **mutable reference**.

### Requirements:

1. The variable must be declared as `mut`
2. Pass the reference as `&mut`
3. The function parameter must accept `&mut`

```rust
fn main() {
    let mut s = String::from("Hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", World"); // ✅ Works now
}
```

---

## Important Rules for Mutable References

### Rule 1: Only One Mutable Reference at a Time

⚠️ **You cannot have multiple mutable references to the same value in the same scope.**

#### ❌ This Will Not Compile:

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s; // ❌ Error: cannot borrow `s` as mutable more than once
println!("{r1}, {r2}");
```

### Rule 2: Cannot Mix Mutable and Immutable References

⚠️ **You cannot have a mutable reference while immutable references exist.**

#### ❌ This Will Not Compile:

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &s; // ❌ Error: cannot borrow as immutable while mutable borrow exists
println!("{r1}, {r2}");
```

---

## Why These Restrictions?

### Preventing Data Races at Compile Time

Rust prevents **data races** by enforcing these rules. A data race occurs when:

1. **Two or more pointers** access the same data at the same time
2. **At least one pointer** is being used to write to the data
3. **No mechanism** is used to synchronize access to the data

💡 **Key insight:** You can have multiple mutable references, but **not at the same time**. They must be used in separate scopes.

---

## Reference Scope

**When does a reference's scope end?**

A reference's scope starts when it's created and ends at its **last use** in the program.

✅ This means you can create new references after the last use of previous ones:

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{r1}, {r2}"); // Last use of r1 and r2

let r3 = &mut s; // ✅ OK: r1 and r2 are no longer used
println!("{r3}");
```

---

## Dangling References

### What is a Dangling Reference?

A **dangling reference** is a pointer that references a location in memory that may have been given to someone else by freeing some memory while preserving a pointer to that memory.

⚡ **Rust's guarantee:** The compiler ensures there are no dangling references. It will make sure that the data will not go out of scope before the reference to that data does.

### Example: Compiler Prevents Dangling References

```rust
fn main() {
    let reference_to_string = dangle();
}

fn dangle() -> &String { // ❌ This will not compile
    let s = String::from("Hello");
    &s
} // s goes out of scope and is dropped here
```

**Why this fails:**

- When `dangle` finishes, `s` goes out of scope and is dropped
- The memory is freed
- But we're returning a reference to that string
- The reference would point to invalid memory (dangling reference)
- ✅ Rust prevents this at compile time

---

## Summary

|Reference Type|Symbol|Can Modify?|Multiple Allowed?|
|---|---|---|---|
|Immutable|`&`|❌ No|✅ Yes|
|Mutable|`&mut`|✅ Yes|❌ No (one at a time)|

### Key Rules:

- ✅ Multiple immutable references are allowed
- ✅ One mutable reference per scope
- ❌ Cannot mix mutable and immutable references
- ❌ Rust prevents dangling references at compile time
- 📌 Reference scope = creation to last use