fn main() {
    println!("Hello, world!");
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFFF!!!!");

    let number_array = [1, 2, 32, 3, 4, 5, 5, 6, 6];
    // let mut index = 0;
    // while index < 8 {
    //     println!("{}", number_array[index]);
    //     index += 1;
    // }
    for element in number_array {
        println!("{}", element);
    }

    println!("====================");

    for number in (1..4).rev() {
        println!("{}", number);
    }

    println!("LIFTOFF!!!!")
}
