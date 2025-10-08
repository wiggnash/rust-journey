Link to resource
[Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability)

Variables are immutable by default but we can make the variables mutable.
what is the meaning of immutable ?
once a value is given to an variable then we cannot change the value

example
```rust
fn main() {
    println!("Hello, world!");
    let x = 5; //assignment 1
    println!("The value of x is : {x}");
    x = 6; // assignment 2
    println!("The value of x is : {x}");
}
```

error
```bash
(base) ➜  variables git:(main) ✗ cargo check
    Checking variables v0.1.0 (/home/wiggnash/Vicky/Projects/learning_programming_languages/rust/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:5:5
  |
3 |     let x = 5;
  |         - first assignment to `x`
4 |     println!("The value of x is : {x}");
5 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
3 |     let mut x = 5;
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` (bin "variables") due to 1 previous error
```

immutability helps in reduce bugs which are caused by us changing the value of an variable unknowingly

how is the mutability helpful ?
syntax : we can make a variable mutable by mentioning `mut`
therefore its value can be changed 

```rust
fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");
}
```


