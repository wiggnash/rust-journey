![[Pasted image 20250212125418.png]]
## 🔍 Why Does Rust Buffer Output?
- Rust buffers standard output for performance reasons
- Rust stores the output in memory before actually displaying it on the terminal : this is called buffer
- The buffer is shown in the terminal/ Flushed when
	- The buffer gets full
	- newline is printed `println!()`
	- flush is explicitly called 

If we are using print ,  we need to manually flush the output to the terminal
```rust
io::stdout.flush().unwrap();
```

This tells the RUST to send everything in the buffer to the screen right now and dont wait for a new line 

## unwrap()
- this is used to handle the potential errors in a simple way
- This is the quick way to extract the values from the Result Type

### working of unwrap()
`read_line()` : This will return an `Result` type
- Ok : The operation in succeeded
- Err : An error occurred

`unwrap()` : will extracts the Ok value if the operation in successful , if any error occures , unwrap() will panic and crashed the program and prints the error message

### 🛠 Why are we using `unwrap()` here?

We are using `.unwrap()` because we expect **reading user input to succeed** in most cases. If an error happens (e.g., stdin is closed, memory issues), the program will **immediately panic** instead of continuing with invalid data.


## .trim()
### without the .trim()
- When we are reading a input from the terminal , it adds new line character at the end 
- `stdin.read_line(&mut input).unwrap();` **reads the user input**, but it also includes the **newline character (`\n`)** at the end.
- When printing `input`, the newline (`\n`) is also printed.
- This causes the output to be **split into two lines** unexpectedly.

### with the .trim()
- `trim()` **removes** any leading/trailing whitespace, including the **newline character (`\n`)**.
- Now, `input` contains only the actual text the user entered.
- This keeps the output on the **same line**.