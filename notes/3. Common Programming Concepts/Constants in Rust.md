## 📚 Resource

[Constants - The Rust Book](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#constants)

---

## 🔐 What are Constants?

**Constants** are always immutable variables that remain valid for the entire duration of the program within their declared scope.

---

## 🔄 Constants vs Variables

|Feature|Constants|Variables|
|---|---|---|
|**Mutability**|Always immutable|Immutable by default, can be mutable with `mut`|
|**Keyword**|`const`|`let`|
|**Type annotation**|Required|Optional (can be inferred)|
|**Scope**|Any scope (including global)|Typically local scope|
|**Naming convention**|UPPER_CASE_WITH_UNDERSCORES|snake_case|
|**Value assignment**|Must be compile-time constant expression|Can be runtime value|

### Key Differences

- ❌ **Cannot use `mut` with constants** - they are always immutable
- ✅ **Type must be explicitly annotated** for constants
- 🌍 **Can be declared in any scope**, including global scope

---

## 📝 Syntax

```rust
const CONSTANT_NAME: Type = value;
```

### Example

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

---

## ⚡ Compile-Time Evaluation

Constants must be set to values that can be determined **at compile time**.

### ✅ Valid Constant Expressions

- Literal values
- Mathematical expressions with literals
- Other constants

### ❌ Invalid for Constants

- Function calls (runtime computation)
- Values computed during program execution
- Results from runtime operations

**Example:**

```rust
// ✅ Valid - evaluated at compile time
const MAX_POINTS: u32 = 100_000;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// ❌ Invalid - cannot use runtime values
// const CURRENT_TIME: u32 = get_current_time(); // ERROR!
```

---

## 📌 Naming Convention

Constant names should follow these rules:

- **ALL UPPERCASE LETTERS**
- **Underscores between words**

**Examples:**

```rust
const MAX_PLAYERS: u32 = 100;
const PI_VALUE: f64 = 3.14159;
const SERVER_TIMEOUT_SECONDS: u32 = 30;
```

---

## Key Takeaways

- Constants are **always immutable** - no `mut` allowed
- **Type annotation is required** for all constants
- Constants are evaluated **at compile time**, not runtime
- Use **UPPER_CASE_WITH_UNDERSCORES** naming convention
- Can be declared in **any scope**, making them useful for global configuration values