struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // creating new instance by using some values from another instance without
    // struct update syntax
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("newemail@gmail.com"),
    //     sign_in_count: 1
    // }

    // with struct update syntax
    let user2 = User {
        // username: String::from("new user name"),
        email: String::from("newemail@gmail.com"),
        ..user1 //this syntax should always comes in the end
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(x, y, z) = origin;
    println!("{x} {y} {z}");

    let Color(x, y, z) = black;
    println!("{x} {y} {z}");

    example_struct_program();
}

fn example_struct_program() {
    example_witout_struct();
    example_with_tuple();
    example_with_struct();
}

fn example_witout_struct() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area_without_struct(width, height)
    );
}

fn area_without_struct(width: u32, height: u32) -> u32 {
    width * height
}

fn example_with_tuple() {
    let rec = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels",
        area_with_tuple(rec)
    );
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn example_with_struct() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area_with_struct(&rec)
    );
}

fn area_with_struct(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}
