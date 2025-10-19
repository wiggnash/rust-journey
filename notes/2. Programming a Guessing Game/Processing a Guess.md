1. ask for user input
2. process the input
3. check that if the input is in the expected form

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

for getting the input from the user and showing the output we need input/output IO library into the scope. this IO library comes from the standard library

bringing the functionalities of an library into the scope is called **prelude**

here as you see the main is the first entry point to the program

fn - syntax for new function
() - to accept the arguments through paramters
{} - shows the start and the end of the program

