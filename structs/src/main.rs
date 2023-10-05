fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user.email = String::from("anotheremail@example.com");

    let user2 = User {
        active: user.active,
        username: user.username,
        email: String::from("another@example.com"),
        sign_in_count: user.sign_in_count,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Using Tuples without named fields to create different types

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    // Example program using structs
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels", area1(rect1));

    let rect1 = Rectangle { width: 30, height: 50,};
    println!("The are of the rectangle is {} square pixels.", area2(&rect1));

    // Adding functionality with Derived Traits

}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

struct AlwaysEqual;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn new_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}