fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("This is a constant : {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let y = 5;
    let y = y + 10; // shadow for the first time

    {
        let y = y * 2; // shadow for the second time
        println!("The value of y in the inner scope is : {y}");
    }

    println!("The value of y is : {y}");

    // exmaple 2 shadowing
    let spaces = "    "; // 4 spaces and type is string
    let spaces = spaces.len(); // type is changed to number

    println!("Spaces : {spaces}");

    // // exmaple 3 : ERROR Code
    // let mut spaces = "    "; // 4 spaces and type is string
    // spaces = spaces.len();

    // println!("Spaces : {spaces}");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Parsed Number {guess}");

    // compound data types
    // 1. tuples
    let sample_tuple = (1.2, 1, 500);
    let (x, y, z) = sample_tuple;

    println!("Tuple elements are x {x} y {y} z {z}");

    let first_element = sample_tuple.0;
    let second_element = sample_tuple.1;
    let third_element = sample_tuple.2;

    println!(
        "First element : {first_element}, Second element : {second_element}, Third element : {third_element}"
    );

    // 2. arrays
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // println!("Month array : {months}"); // showing some error need to find out why
    let fixed_len_array: [i32; 5] = [1, -3, 4, 8, 2];
    let predefined_array = [3; 5];
}
