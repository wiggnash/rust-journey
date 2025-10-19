Links to the Resource
[Generating a Random Number](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#generating-a-random-number)

```rust
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {secret_number}");
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

```rust
use rand::Rng;
```

Rng this is a trait and this is what will help us generate random numbers , therefore this must be in scope

```rust
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {secret_number}");
```

```rust
rand::thread_rng()
```

this is the particular random number generator that we are going to use 
this random number is seeded by the operating system

```rust
.gen_range(1..=100);
```

this method defined by the Rng trait
we need to define the range using an expression

start..=end is the syntax of the range expression
which is inclusive of both lower and upper bound : therefore the range is from 1 - 100