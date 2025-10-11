**Reference:** [Control Flow - The Rust Programming Language](https://doc.rust-lang.org/book/ch03-05-control-flow.html#control-flow)

---

## If Expressions

**`if`** expressions allow you to branch code based on conditions.

### Rules

- The condition **must** evaluate to a **boolean** (`true` or `false`)
- Rust **does not** automatically convert non-boolean values to boolean (unlike JavaScript)
- If the condition is not boolean, you'll get a compile error

### Basic Example

```rust
fn main() {
    let number = 3;
    
    if number < 5 {
        println!("Condition was true!");
    } else {
        println!("Condition was false");
    }
}
```

### Multiple Conditions with `else if`

```rust
fn main() {
    let number = 6;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

---

## Repetition with Loops

Rust has **three kinds of loops**:

1. **`loop`** - Infinite loop until explicitly stopped
2. **`while`** - Loop while condition is true
3. **`for`** - Loop over a collection

---

## `loop` - Infinite Loop

Repeats code **indefinitely** until explicitly told to stop.

### Control Keywords

- **`break`** - Exit the loop immediately
- **`continue`** - Skip to the next iteration without completing current iteration

### Basic Example

```rust
fn main() {
    let mut counter = 0;
    
    loop {
        counter += 1;
        
        if counter == 10 {
            break;  // Exit the loop
        }
    }
    
    println!("Counter: {counter}");
}
```

---

## Returning Values from Loops

You can **return a value** from a loop by placing it after the `break` statement.

💡 **Use case:** Retry operations that might fail (e.g., checking if a thread completed)

### Example

```rust
fn main() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;  // Return value from loop
        }
    };
    
    println!("The result is {result}");  // Prints: 20
}
```

### `break` vs `return`

- **`break`** - Exits the current loop
- **`return`** - Exits the entire function

---

## Loop Labels

You can **label loops** and reference them in `break` or `continue` statements for nested loops.

### Syntax

- Loop labels start with a single quote: `'label_name:`
- Use `break 'label_name` to break a specific loop

### Example

```rust
fn main() {
    let mut count = 0;
    
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            
            if remaining == 9 {
                break;  // Breaks inner loop only
            }
            if count == 2 {
                break 'counting_up;  // Breaks outer loop
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    
    println!("End count = {count}");
}
```

---

## `while` - Conditional Loop

Runs **while a condition is true**. Stops when the condition becomes **false**.

### Example

```rust
fn main() {
    let mut number = 3;
    
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    
    println!("LIFTOFF!!!!");
}
```

**Output:**

```
3!
2!
1!
LIFTOFF!!!!
```

---

## `for` - Collection Loop

Loop over elements in a **collection** or **range**.

### Advantages

- ⚡ **Faster** than `while` in Rust (no condition checking overhead)
- 🛡️ **Safer** - no risk of index errors
- ✅ More concise and readable

### Looping Over Arrays

```rust
fn main() {
    let array = [1, 2, 3, 4, 5];
    
    for element in array {
        println!("the value is {element}");
    }
}
```

### Looping Over Ranges

```rust
fn main() {
    // Count from 1 to 3
    for number in 1..4 {
        println!("{number}!");
    }
    
    // Count down from 3 to 1
    for number in (1..4).rev() {
        println!("{number}!");
    }
    
    println!("LIFTOFF!!!!");
}
```

**Output:**

```
3!
2!
1!
LIFTOFF!!!!
```

---

## 🔑 Key Takeaways

|Loop Type|Use Case|Stop Condition|
|---|---|---|
|**`loop`**|Infinite loop, retry operations|Explicit `break`|
|**`while`**|Loop with condition|Condition becomes `false`|
|**`for`**|Iterate over collections/ranges|End of collection|

---

## 💡 Best Practices

- Use **`for`** when iterating over collections (safest and fastest)
- Use **`while`** when you have a specific condition to check
- Use **`loop`** when you need an infinite loop or want to return values
- Use **loop labels** for nested loops to control which loop to break/continue
- **`if` conditions must be boolean** - Rust won't auto-convert like JavaScript

---

## ⚡ Performance Note

**`for` loops are faster than `while` loops** in Rust because:

- No runtime condition checking
- Compiler can optimize better
- No risk of off-by-one errors