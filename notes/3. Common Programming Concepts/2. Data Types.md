Rust is a **statically typed** language : it must know the types of all variable at the compile time

Most of the time the compiler can infer the type but when we are converting from one type to another type , we have to add the type annotation

> Every value in Rust is of a certain data type , therefore RUST knows with what kinda of data it is working with

## Types of Data types
1. Scalar 
2. Compound

## Scalar Types
These types represent a single value

### There are 4 primary scalar types
1. Integers
2. Float
3. Booleans
4. Characters

#### Integers
These are numbers without fractions , the numbers here are the size of this type

There are two types of integers
1. Signed Integers ( i ) => Numbers which will have its sign , therefore it can contain the negative numbers also. These numbers are stored using 2's complement representation
	1. default value of integers is `i32`
2. Unsigned Integers ( u ) => Numbers which can be only be positive


![[Pasted image 20250213213757.png]]

- Each signed variant can store numbers from -(2^n - 1) to 2^n - 1 - 1 inclusive, where _n_ is the number of bits that variant uses. 
- So an `i8` can store numbers from -(2^7) to 2^7 - 1, which equals -128 to 127. 
- Unsigned variants can store numbers from 0 to 2^n - 1, so a `u8` can store numbers from 0 to 2^8 - 1, which equals 0 to 255.

##### Must Read Important Topic : [Integer Overflow](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow)

#### Floating Point Type
Types of floating point numbers and these are signed numbers
1. f32 => 32bits floating number 
2. f64 => 64 bits floating numbers `default value`

#### Boolean
1. true
2. false

-> size is one byte , type is `bool`

#### Character Type
This is the most primitive alphabetic type

Unlike some languages where `char` is just an 8-bit (1-byte) ASCII character (like in C or JavaScript), Rust’s `char` type is **32 bits (4 bytes)** and can represent any **Unicode scalar value**

Character type `char` is represented as single quotes

## Compound Types
-> Group of multiple values into one type

### Two Primitive Compound Types
1. Arrays
2. Tuples

#### Tuples
-> Tuple can store multiple values of different types into a single type
-> Tuples have fixed length : Cannot grow or shrink in size
-> access the elements using dot operator and write the index of the element we want to access

`tuple without any value is called unit => empty value or empty return type`


```rust
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructure a tuple
    let (x, y, z) = tup;

    println!("The values are :{x} {y} {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("First value (i32): {}", five_hundred);
    println!("Second value (f64): {}", six_point_four);
    println!("Third value (u8): {}", one);
```

#### Arrays
-> Collection of multiple values, every element in an array must be the same type. We can use this when we know the number of elements will not need to change
-> Array in rust is fixed length
-> Array will allocate the data in the stack
-> cannot access the array elements out of bound

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
```
#### Vector Type
-> Collection of multiple values
-> Size is dynamic , therefore the size can grow or shrink

## Display and Debug Trait
The reason Rust requires `{:?}` to print arrays (and some other complex types)
=> because **Rust's `println!` macro uses different formatting traits**, and **not all types implement `Display` by default**. 

![[Pasted image 20250213223048.png]]

