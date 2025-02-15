fn main() {
    println!("Hello, world!");

    let string = "hello";
    println!("This is &str type : {string}");

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    {
        let s = String::from("Hello"); // s is valid from here

        println!("{s}");
    } // the scope is over here and s is no longer valid

    let x = 5;
    let y = x;

    println!("x:{x} y:{y}");

    let s1 = String::from("Hello");
    let s2 = s1;

    println!("s2: {s2}");

    let mut s = String::from("hello");
    println!("{s}");
    s = String::from("Vigneshwar");

    println!("{s}");
}
