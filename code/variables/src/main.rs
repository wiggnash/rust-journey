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
}
