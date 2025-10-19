## Clone

What if we want to copy the data which is present in the heap , which means what if we want to do deep copy ?

we can use this method called `clone`

```rust
fn main() {
    let string_1 = String::from("Hello");
    let copy_string_1 = string_1.clone();
    
    println!("String 1 : {string_1} , String 2 : {copy_string_1}");
}
```

## Copy

when we are interacting with the simple data types , where the size is fixed , which means that we know the size at the compile time.
this data is stored on the stack , therefore copying this is possible and quick

`Copy` trait

this trait can be used on the variables which are stored on the stack 
`Copy` trait and `Drop` trait cannot exist at the same time

Therefore we need to understand what are the types which implement the `Copy` trait ?
Any group of simple scalar values can implement `Copy` trait

Example

1. all integer types
2. Boolean types
3. floating types
4. char type
5. tuples


