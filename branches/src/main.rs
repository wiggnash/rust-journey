fn main() {
    println!("Hello, world!");

    let number = 3;

    if number < 5 {
        println!("Condition was true!");
    } else {
        println!("Condition was false");
    }

    let condition = false;
    let number_based_on_condition = if condition { 5 } else { 6 };

    println!("number based on condition : {number_based_on_condition}")
}
