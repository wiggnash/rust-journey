```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

```rust
println!("You guessed: {guess}");
```

{guess} = this holds the value in the place , therefore we can pass the variable name inside the curly brackets