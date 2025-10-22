## 📌 What are Slices?

**Slices** let us reference a contiguous sequence of elements in a collection. A slice is a **reference** (not owned data), so it follows borrowing rules.

> 💡 **Idiomatic Rust**: Functions should not take ownership of their arguments unless they need to.

---

## 🔍 Problem: Finding the First Word

**Task**: Write a function that takes a string of words separated by spaces and returns the first word. If no space exists, return the entire string.

### ❌ Solution Without Slices

rust

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    s.len()
}
```

**How it works:**

1. Convert string to bytes using `as_bytes()`
2. Use `iter()` to iterate over each element
3. Use `enumerate()` to get `(index, reference_to_element)` tuples
4. Check for space using byte literal syntax `b' '`
5. Return index of space, or length if no space found

**Problem with this approach:**

- Returns an index that's tied to the string's state
- If the string goes out of scope or changes, the index becomes meaningless
- The index and string are decoupled—dangerous!

---

## ✅ String Slices

Slices reference a contiguous sequence of elements without ownership.

### Basic Syntax

rust

```rust
fn main() {
    let s = String::from("Hello World");
    let hello = &s[0..5];   // "Hello"
    let world = &s[6..11];  // "World"
    
    println!("first word {hello}");
    println!("second word {world}");
}
```

**Range syntax**: `[starting_index..ending_index]`

- `starting_index` is inclusive
- `ending_index` is **exclusive** (one past the last character)
- `0..5` means indices 0 through 4
- `6..11` means indices 6 through 10

**Shortcuts:**

- `..5` → start from beginning (same as `0..5`)
- `6..` → go to the end (same as `6..length`)

### Memory Structure of a Slice

A slice stores two values:

1. **Pointer**: Address of the starting index
2. **Length**: Number of elements to include

### ✅ Better Solution Using Slices

rust

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    
    &s[..]
}
```

**Key improvement:**

- Returns a string slice `&str` that references the actual data
- The slice is tied to the original string's lifetime
- Safer: if the string is dropped, the slice can't be used (enforced by borrow checker)

**⚠️ Borrowing Rule**: If you have an immutable reference to something, you cannot take a mutable reference at the same time.

## String Literals & String Types 

### 1. What is a Binary/Executable?

When you compile Rust code, the compiler converts your human-readable code into **machine code** (instructions the CPU understands). This compiled output is saved as a **binary** or **executable** file.

**Key difference from JavaScript/Python:**

- JavaScript/Python: Interpreted or JIT-compiled at runtime
- Rust: Compiled ahead of time into a standalone executable file

---

### 2. Where String Literals Live

```rust
let s = "Hello, world!";
```

When you write a string literal in your code, that text gets **baked into your binary file** during compilation. It's stored in a special **read-only section** of the binary.

**Important:** The string data exists in your executable file _before_ your program even runs. No memory allocation needed at runtime!

---

### 3. What is `&str`?

`&str` (string slice) is a reference type that points to string data that already exists somewhere. It contains two pieces of information:

1. **A pointer/reference** - where the string data is located in memory
2. **A length** - how many bytes long the string is

```rust
let s = "Hello, world!";
// s is of type &str
// It points to "Hello, world!" in the binary
// It knows the length is 13 bytes
```

---

### 4. Why String Literals are Immutable

String literals are stored in the **read-only section** of memory. This is enforced by the operating system.

**What happens if you try to modify read-only memory?**

- Your program crashes with a **segmentation fault** (segfault)
- The OS immediately terminates your program

This is why `&str` from string literals _must_ be immutable - it's a physical reality, not just a design choice.

---

### 5. Making the Variable Binding Mutable vs Making the Data Mutable

### ✅ This works - changing which string the variable points to:

```rust
let mut s = "Hello, world!";
s = "Goodbye!";  // Just moving the pointer to a different string literal
```

**What's happening:**

```
Binary (read-only):
  "Hello, world!" ← at address 0x1000
  "Goodbye!"      ← at address 0x2000

Stack:
  s → initially points to 0x1000
  s → after reassignment points to 0x2000
```

Both strings exist in the binary. We're just changing which one `s` points to.

### ❌ This fails - trying to modify the actual string data:

```rust
let mut s = "Hello, world!";
s.push_str(" How are you?");  // Compile error!
```

**Error:** `no method named 'push_str' found for reference '&str'`

**Why it fails:**

- `&str` is just a view/slice into existing data
- You can't grow or modify data that's baked into the binary
- The read-only section can't change at runtime

---

### 6. When You Need a Mutable, Growable String: `String`

When you need to modify or grow a string, use the `String` type. It **owns its data** and stores it on the **heap**.

```rust
// String literal - lives in binary, immutable
let s1: &str = "Hello, world!";

// String - owns its data on heap, mutable and growable
let mut s2: String = String::from("Hello, world!");
s2.push_str(" How are you?");  // ✅ Works!
```

---

### 7. Two Types of `&str` Slices

`&str` can point to two different places:

#### A) String literals in the binary (read-only)

```rust
let s: &str = "Hello, world!";  // Points to binary
```

#### B) Borrowed slices of a `String` on the heap

```rust
let s: String = String::from("Hello, world!");
let slice: &str = &s[0..5];  // Points to "Hello" in the heap
```

---

### 8. Quick Comparison: `&str` vs `String`

|Feature|`&str`|`String`|
|---|---|---|
|**Ownership**|Borrowed reference (doesn't own data)|Owns its data|
|**Memory location**|Can point to binary or heap|Always on heap|
|**Mutability**|Immutable|Can be mutable|
|**Size**|Fixed size (can't grow)|Growable|
|**Use case**|Reading/viewing strings|Building/modifying strings|

---

### 9. Summary

**String literals:**

- Are embedded in your compiled binary at compile time
- Live in read-only memory
- Are represented as `&str` (a slice pointing to that data)
- Cannot be modified (attempting to do so would crash your program)

**The `&str` type:**

- Is a reference/pointer + length
- Points to string data that exists elsewhere
- Is always immutable
- Can point to data in the binary OR borrowed from a `String`

**The `String` type:**

- Owns its data on the heap
- Can be mutated and grown
- Use this when you need to build or modify strings at runtime

---

### 10. Mental Model Coming from JavaScript/Python

**In JavaScript/Python:**

```javascript
let s = "Hello, world!";
s = s + " How are you?";  // Creates new string, runtime handles memory
```

You don't think about where strings live. The runtime manages everything.

**In Rust, you have explicit control:**

```rust
// Option 1: Just a view (can't modify)
let s: &str = "Hello, world!";

// Option 2: Owned, growable (can modify)
let mut s: String = String::from("Hello, world!");
s.push_str(" How are you?");
```

Rust makes you think about:

- Who owns the data?
- Where does it live (stack/heap/binary)?
- Can it be modified?
