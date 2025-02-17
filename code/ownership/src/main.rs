// fn main() {
//     println!("Hello, world!");

//     let string = "hello";
//     println!("This is &str type : {string}");

//     let mut s = String::from("hello");
//     s.push_str(", world!");
//     println!("{s}");

//     {
//         let s = String::from("Hello"); // s is valid from here

//         println!("{s}");
//     } // the scope is over here and s is no longer valid

//     let x = 5;
//     let y = x;

//     println!("x:{x} y:{y}");

//     let s1 = String::from("Hello");
//     let s2 = s1;

//     println!("s2: {s2}");

//     let mut s = String::from("hello");
//     println!("{s}");
//     s = String::from("Vigneshwar");

//     println!("{s}");
// }

// fn main() {
//     let copy_integer = 12;
//     let hello_string = String::from("Hello String");
//     transfer_ownership(hello_string);
//     makes_copy(copy_integer);
// }

// fn transfer_ownership(hello_string: String) {
//     println!("{hello_string}");
// }

// fn makes_copy(copy_integer: i32) {
//     println!("{copy_integer}");
// }

// fn main() {
//     let name = String::from("Vigneshwar");
//     let my_name = greeting(name);

//     println!("Welcome to my course {my_name}");
// }

// fn greeting(name: String) -> String {
//     println!("Hello {name}");
//     name
// }

// This is not possible and throw an error

// fn main() {
//     let s1 = String::from("Hello");
//     let length = get_string_length(s1);
//     println!("Length of the {s1} is {length}");
// }

// fn get_string_length(string: String) -> i32 {
//     string.len()
// }

// fn main() {
//     let s1 = String::from("Hello");
//     let (s2, length) = get_string_length(s1);

//     println!("Length of the {s2} is {length}");
// }

// fn get_string_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

// this will throw an error becaus of immutable reference
// fn main() {
//     let s1 = String::from("Hello");
//     let length = get_string_length(&s1);
//     println!("Length of the string: {s1} is {length}");
// }

// fn get_string_length(s: &String) -> usize {
//     s.push_str(", World!");
//     s.len()
// }

// mutable reference
// fn main() {
//     let mut s1 = String::from("Hello");
//     let length = get_string_length(&mut s1);
//     println!("Length of the string: {s1} is {length}");
// }

// fn get_string_length(s: &mut String) -> usize {
//     s.push_str(", World!");
//     s.len()
// }

// limitation of mutable reference
fn main() {
    let mut s1 = String::from("Hello");
    let reference_one = &mut s1;
    let reference_two = &mut s1;
    println!("Mutable reference : {reference_one} and {reference_two}");
}
