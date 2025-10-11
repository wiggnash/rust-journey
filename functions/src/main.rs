fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_parameter(7);
    print_labeled_measurement(5, 'h');

    let five = five();
    println!("Function returned value : {five}");
}

fn another_function() {
    println!("Another function is called");
}

fn another_function_with_parameter(x: i32) {
    println!("Another function with parameter : {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
}

fn five() -> i32 {
    5;
}
