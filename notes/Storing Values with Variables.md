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
let mut guess = String::new();
```

**let** statement is used to create variables

in Rust **variables are immutable** by default : once we given a value to an variable , the value will not change or cannot be changed.

if you want to make the variable as mutable : then use this keyword **mut** before creating an variable

**String** type is from the standard library and it is growable and UTF-8 encoded

**: :** : we can add this to any of the data type
this means it is the associated function of the String type

what is an associate function ?
it is the function , these functions are implemented on a type

**: :new** : this associated function will be there in all the data types , its like creating an copy of the already present ML

**String::new()** : this is a function that returns an new instance of String

in one line

we have created a mutable variable that is currently bound to a new , empty instance of a String data type.

