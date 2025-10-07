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

from the standard input/output library **use std::io**

**stdin** is used for taking the input

```rust
io::stdin().read_line(&mut guess)
```

**io::stdin()** this function returns an instance of **std::io::Stdin** which handles to the standard input for the terminal

why we are using &mut guess ?
read_line() method takes the input from the terminal and append to the string variable instead of overwritting its value
therefore the string have to be mutable for us to change its value

& - this is reference
which gives you a way to let multiple parts of your code to access one piece of data without needing to copy the data into the memory multiple times.
for now just know that references are immutable by default
to make the reference mutable we need to write it as &mut 