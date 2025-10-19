Link to the resource : [Scope and Assignment](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#scope-and-assignment)

## Assignment new value to the variable present in the heap memory

```rust
fn main() {
    let mut string_1 = String::from("Hello");
    println!("string : 1 {string_1} ");
    
    string_1 = String::from(", World!");

    println!("string : 1 {string_1} ");
}
```

When we assign a completely new value to an existing variable , Rust will call `drop` and free the original value's memory 

this is because at this line `string_1 = String::from(", World!");` 
there is nothing point to the value "Hello" which is present in the heap's memory , therefore this goes out of scope. Rust will run drop function on it and its memory will be freed 


