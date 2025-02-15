fn main() {
    let mut x = 5;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("This is a constant value {THREE_HOURS_IN_SECONDS}");
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {y}");
    }
    println!("The value of y is : {y}");

    let spaces = "     "; // string type
    let spaces = spaces.len(); // changed to number with the help of shadowing
    println!("the number of spaces is {spaces}");

    let mut string_type: &str = "Vigneshwar";
    println!("This is string type: {string_type}");

    string_type = "Deivasigamani";
    println!("This is string type: {string_type}");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Type anotated when converting from one type to another : {guess}");

    // floating point numbers
    let floating_number_64 = 2.32;
    let floating_number_32: f32 = 3.13;

    println!(
        "Different types of floats : {} {}",
        floating_number_64, floating_number_32
    );

    // Numeric operations
    // addition
    // addition
    let sum = 5 + 10;
    println!("Sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("Difference: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("Product: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("Quotient: {}", quotient);

    let truncated = -5 / 3; // Results in -1
    println!("Truncated Division: {}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("Remainder: {}", remainder);

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("Boolean : {t} {f}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("Character c: {}", c);
    println!("Character z: {}", z);
    println!("Heart Eyed Cat Emoji: {}", heart_eyed_cat);

    // Compounds Types
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

    let array_rust = [1, 2, 3, 43, 5];
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
    println!("Number Array : {:?}", array_rust);
    println!("Month array : {:?}", months);
}
