Link to the resource
[Comparing the Guess to the Secret Number](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number)

```rust
use rand::Rng;
use std::cmp::Ordering;
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

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Less => println!("Too Small!"),
    }
}

```


The `Ordering` type is another enum and has the variants 
1. `Less`,
2. `Greater`,  
3. `Equal`.

`cmp` this method compares two values , therefore can be used on anything which can be compared
what does it take as input ?
it takes the reference to what we want to compare and returns a variant of the `Ordering` enum

`match` : this is used to decide what to do next based on the Ordering variant

- this is made up of arms and these arms consists of two things , which is a `pattern` to match and logic which have to be executed after this match

## Issue or Error we will run into

```rust
    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Less => println!("Too Small!"),
    }
```

**error**

```bash
(base) ➜  guessing_game git:(main) cargo check
    Checking guessing_game v0.1.0 (/home/wiggnash/Vicky/Projects/learning_programming_languages/rust/guessing_game)
error[E0308]: mismatched types
   --> src/main.rs:19:21
    |
 19 |     match guess.cmp(&secret_number) {
    |                 --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
    |                 |
    |                 arguments to this method are incorrect
    |
    = note: expected reference `&String`
               found reference `&{integer}`
note: method defined here
   --> /home/wiggnash/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:979:8
    |
979 |     fn cmp(&self, other: &Self) -> Ordering;
    |        ^^^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` (bin "guessing_game") due to 1 previous error
```

Important point : Rust has a strong , static type system and also type inference.

here the secret_number is a number type
different types of numbers types present in rust
1. i32 : can store value between 1 - 100 : 32 bit number : if we do not specify it defaults to i32
2. u32 : unsigned 32 bit number
3. i64 : 64 bit number
4. etc

The main reason for the error : We cannot compare two different data types
guess : is a string ( typed by the user )
secret_number : this is a number ( generated )

therefore we convert the string type into a number , which means we should convert the type of the guess variable

```rust
use rand::Rng;
use std::cmp::Ordering;
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

    let guess: u32 = guess.trim().parse().expect("Please type a number");
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Less => println!("Too Small!"),
    }
}

```

Variable shadowing is allowed in RUST : Rust allows us to shadow the previous value with a new one , `Shadowing` lets us reuse the variable name rather than creating two different variables

where it will be used ? this will be used whenever we want to convert value from one type to another type

`trim` : this methods removes any whitespace at the beginning and end , we need to do this before converting any string to an number
it also removes the insertion of \n at the end of the string , this gets inserted whenever we enter a new value in the terminal and press enter 

`parse` : converts string to another type . in this example string gets converted to an number
this will Result enum and as we know it contains two variants
1. Ok
2. Err

if its Ok : expect  will return the value present in the Ok
if its Err : expect will break the program and returns the error message present in the Err variant

