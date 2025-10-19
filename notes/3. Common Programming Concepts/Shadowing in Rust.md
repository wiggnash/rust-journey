## 📚 Resource

[Shadowing - The Rust Book](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)

---

## 🎭 What is Shadowing?

**Shadowing** allows you to declare a new variable with the same name as a previous variable. The first variable is "shadowed" by the second variable, meaning the second variable's value is what the compiler sees.

---

## 📋 Basic Example

```rust
fn main() {
    let y = 5;
    let y = y + 10; // Shadow for the first time (y = 15)
    
    {
        let y = y * 2; // Shadow for the second time (y = 30)
        println!("The value of y in the inner scope is : {y}");
        // Output: The value of y in the inner scope is : 30
    }
    
    println!("The value of y is : {y}");
    // Output: The value of y is : 15
}
```

**Key Point:** The inner scope shadow is only valid within that scope. Once the scope ends, the previous shadowed value is restored.

---

## 🔄 Shadowing vs `mut`

### How Shadowing Differs from Mutability

|Feature|Shadowing (`let`)|Mutability (`mut`)|
|---|---|---|
|**Creates new variable**|✅ Yes|❌ No (same variable)|
|**Can change type**|✅ Yes|❌ No|
|**Requires `let` keyword**|✅ Yes|❌ No|
|**Final immutability**|✅ Can be immutable after transformations|⚠️ Stays mutable|

### 💡 Benefits of Shadowing

- Perform **transformations** on a value while keeping the variable **immutable** after completion
- **Change the type** of a variable while reusing the same name
- Create a **new variable** with the same name, effectively replacing the old one

---

## 🔀 Type Transformation with Shadowing

### ✅ Valid: Shadowing Allows Type Change

```rust
fn main() {
    let spaces = "    "; // Type: &str (string slice)
    let spaces = spaces.len(); // Type: usize (number)
    println!("Spaces : {spaces}");
    // Output: Spaces : 4
}
```

**Why it works:** Each `let` creates a **new variable**, so the type can change.

---

### ❌ Invalid: `mut` Does NOT Allow Type Change

```rust
fn main() {
    let mut spaces = "    "; // Type: &str (string slice)
    spaces = spaces.len(); // ERROR: trying to assign usize to &str
    println!("Spaces : {spaces}");
}
```

**Error Output:**

```bash
error[E0308]: mismatched types
  --> src/main.rs:30:14
   |
29 |     let mut spaces = "    "; // 4 spaces and type is string
   |                      ------ expected due to this value
30 |     spaces = spaces.len();
   |              ^^^^^^^^^^^^ expected `&str`, found `usize`
```

**Why it fails:** With `mut`, you're modifying the **same variable**, which must maintain its original type.

---

## 📌 Key Differences Summary

### Shadowing with `let`

- Creates a **new variable** with the same name
- Allows **type changes**
- Variable is **immutable** after the final transformation
- Useful for transforming data while keeping names meaningful

### Mutation with `mut`

- Modifies the **same variable**
- **Cannot change type**
- Variable remains **mutable** throughout
- Useful for values that need continuous updates

---

## Key Takeaways

- **Shadowing** = declaring a new variable with the same name using `let`
- Each shadowing creates a **completely new variable**
- Shadowing allows **type transformation**, `mut` does not
- Shadowing enables transformations while maintaining **final immutability**
- Use shadowing when you need to **reuse a name** with different types or immutable transformations
- Use `mut` when you need to **continuously update** the same variable