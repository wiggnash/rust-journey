**Reference:** [The Character Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type)

---

## Overview

The **`char`** type is Rust's most primitive alphabetic type.

---

## Character Representation

|Syntax|Type|Example|
|---|---|---|
|**Single quotes (`'`)**|Character|`'a'`, `'Z'`, `'😊'`|
|**Double quotes (`"`)**|String|`"hello"`, `"Rust"`|

⚠️ **Important:** Characters use `'single quotes'`, strings use `"double quotes"`

---

## Size and Encoding

- **Type keyword:** `char`
- **Size:** 4 bytes (32 bits)
- **Encoding:** Unicode Scalar Value (not ASCII)

### 💡 Why 4 Bytes?

Rust's `char` represents a **Unicode Scalar Value**, which means it can store:

- Basic ASCII characters (`'a'`, `'1'`)
- Accented characters (`'ñ'`, `'é'`)
- Emoji (`'😊'`, `'🚀'`)
- Characters from any language (`'中'`, `'あ'`, `'א'`)

---

## Example

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    let chinese_char = '中';
}
```

---

## 🔑 Key Takeaways

- `char` uses **single quotes**, strings use **double quotes**
- Size: **4 bytes** (32 bits)
- Supports **Unicode**, not just ASCII
- Can represent characters from any language and emoji