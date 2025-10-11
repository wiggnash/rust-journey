fn main() {
    println!("Hello, world!");
    // loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result value is {result}");

    // named loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // while
    println!("===================");
    let mut while_number = 3;

    while while_number != 0 {
        println!("{while_number}!");
        while_number -= 1;
    }

    println!("LIFTOFF!!!!");

    // for
    println!("===================");
    let array = [1, 2, 3, 4, 5];

    for element in array {
        println!("the value is {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("Lift offff!!!!");
}
