## Overview

Integers are numbers without any fractional component.

---

## Types of Integer Types

Rust provides two categories of integer types:

1. **Unsigned** – Cannot hold negative numbers
2. **Signed** – Can hold negative numbers; stored using [2's complement](https://en.wikipedia.org/wiki/Two%27s_complement) representation

### Integer Type Table

|Length|Signed|Unsigned|
|---|---|---|
|8-bit|`i8`|`u8`|
|16-bit|`i16`|`u16`|
|32-bit|`i32`|`u32`|
|64-bit|`i64`|`u64`|
|128-bit|`i128`|`u128`|
|architecture dependent|`isize`|`usize`|

---

## Range of Integer Types

### Example: 8-bit Integer

- 2^8 = 256 possible values
- **Unsigned (`u8`)**: 0 to 255
- **Signed (`i8`)**: -128 to 127

### General Formula

- **Signed**: −(2^(n − 1)) to 2^(n − 1) − 1
    - Where `n` is the number of bits
- **Unsigned**: 0 to 2^n − 1

**Example:**  
A `u8` can store numbers from 0 to 2^8 − 1, which equals 0 to 255.

---

## Integer Literals

Integers can be written in multiple formats:

|Number Literals|Example|
|---|---|
|Decimal|`98_222`|
|Hex|`0xff`|
|Octal|`0o77`|
|Binary|`0b1111_0000`|
|Byte (`u8` only)|`b'A'`|

💡 **Tip:** Use underscores (`_`) as visual separators for readability (e.g., `98_222`).

---

## Integer Overflow ⚠️

**Integer overflow** occurs when you try to store a value larger than the maximum limit of a type.

### Behavior in Debug Mode

- Rust compiler checks for integer overflow during compilation
- If overflow is detected, the program will **panic** at runtime
- The program exits without producing an error message

### Behavior in Release Mode

- Rust compiler does **not** include integer overflow checks
- When overflow occurs, Rust performs **2's complement wrapping**
- The value "wraps around" to the minimum value the type can hold

📌 **Example:**  
If you try to store `257` in a `u8` (max = 255), it wraps around to `1`.

```
257 mod 256 = 1
```

### Handling Integer Overflow

Rust provides methods to explicitly handle integer overflow:

- `wrapping_*` – Wraps on overflow
- `checked_*` – Returns `None` on overflow
- `overflowing_*` – Returns value and a boolean indicating overflow
- `saturating_*` – Clamps at min/max bounds

✅ Use these methods when you need explicit control over overflow behavior.