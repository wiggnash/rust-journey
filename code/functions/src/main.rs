fn main() {
    println!("Hello, world!");
    another_function(32, 'V');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}");

    let five_returned_value = return_five();
    println!("The value is {five_returned_value}");
}

fn return_five() -> i32 {
    5 // this is the final expression , therefore function returns this value
}

fn another_function(x: i32, unit_label: char) {
    println!("Another Function!");
    println!("The value of x is {x} : {unit_label}");
}
