link to resource :  [Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html#control-flow)

## If expressions

depending on a condition we can branch our code
this condition should result in a boolean value
if its not a boolean then we will get an error

In javascript , it will automatically try to convert the non boolean to boolean

example
```rust
fn main() {
    println!("Hello, world!");

    let number = 3;

    if number < 5 {
        println!("Condition was true!");
    } else {
        println!("Condition was false");
    }
}

```

multiple if else

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

## Repetition with Loops

### Types of loops

1. loop
2. while
3. for

### Loop

keep repeating the code over and over again until we explicitly tell it to stop

`break` : to break the loop
`continue` : to go for the next iteration in the loop without completing the remanining logic present in the iteration

### Returning values from loops

`loop` is used to retry an operation you know might fail
ex : to check if a thread has completed its job , therefore in this case we might need to pass the result of that operation out of the loop

we can do this by returning the value after the break

any expression or value we give after the break , that gets returned to the loop

we can either break or return
`break` this will break the loop and exit the current loop
`return` this will exit the current function

### Loop labels

we can give names to the loop
using these loop we can break or continue that loop specifically

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

```

### while

while the condition holds true , then the loop runs
when the condition is false , then the loop stops

```rust
fn main() {
    println!("===================");
    let mut while_number = 3;

    while while_number != 0 {
        println!("{while_number}!");
        while_number -= 1;
    }

    println!("LIFTOFF!!!!");
}
```

### for

it is used to loop over a grouping
its faster than while in the rust because there is no condition to compare

```rust
fn main() {
    // for
    println!("===================");
    let array = [1, 2, 3, 4, 5];

    for element in array {
        println!("the value is {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("Lift offff!!!!");
}

```
