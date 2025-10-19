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
.expect("Failed to read line");
```



**read_line()** this takes the input from the user in terminal and appends it to the variable
and also it returns something called **Result**

## Result

Result is an enumeration or enum
which means there will be set of possible states/variants and result can take any one value from that

Purpose : to encode the error handling information

### variants of Result

1. Ok : Indicates that the operation is successful and it contains the successfully generated value
2. Err : indicates that the operation failed and it contains the information about how and why the operation failed

Result type has method defined and one of the method is called expect

what is the advantage of expect ?
If the Result is an Err value then the expect will cause the program to crash and display the message that you passed as an argument to expect

If the Result is Ok , then expect will take the return value of Ok and returns that
