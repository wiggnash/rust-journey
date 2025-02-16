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

fn main() {
    let name = String::from("Vigneshwar");
    let my_name = greeting(name);

    println!("Welcome to my course {my_name}");
}

fn greeting(name: String) -> String {
    println!("Hello {name}");
    name
}
