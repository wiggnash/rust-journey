Every value is of a certain data type
Therefore RUST know how to work with that data

Rust is a statically typed language : we need to mention all the types of the variables before the compile time , compiler can also infer the type based on what data is being used and how we use it . 

it is good practise to mention the types when a variable type is changed from one form to another form

```rust
let guess: u32 = "42".parse().expect("Not a number!");
println!("Parsed Number {guess}");
```

if we dont mention the type there we will get an error something like this

```bash
(base) ➜  variables git:(main) ✗ cargo check
    Checking variables v0.1.0 (/home/wiggnash/Vicky/Projects/learning_programming_languages/rust/variables)
error[E0284]: type annotations needed
  --> src/main.rs:34:9
   |
34 |     let guess = "42".parse().expect("Not a number!");
   |         ^^^^^        ----- type must be known at this point
   |
   = note: cannot satisfy `<_ as FromStr>::Err == _`
help: consider giving `guess` an explicit type
   |
34 |     let guess: /* Type */ = "42".parse().expect("Not a number!");
   |              ++++++++++++

For more information about this error, try `rustc --explain E0284`.
error: could not compile `variables` (bin "variables") due to 1 previous error
```

# Types in RUST

1. [[Scalar types in RUST]]
2. [[Compound types in RUST]]
3. 