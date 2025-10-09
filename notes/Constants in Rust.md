[Constants](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#constants)

Constants are always immutable variables and these are valid for the entire time a program runs within the scope in which they are declared

Difference between Constant and variables ?

it is not allowed to use `mut` with constants

syntax
use `const` keyword define the constants
type of the constant have to be mentioned
constants are declared in any scope : including global scope
name of the constant should be in upper case and underscore between the words

note : we cannot some resultant value which is generated using the run time , we must define the const value well in advance
we can set these constants using an expression

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3
```

at the compile time , this constant is evaluated 