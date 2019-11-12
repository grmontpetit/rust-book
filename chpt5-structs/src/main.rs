use std::fmt;

fn main() {
    defining_and_instanciating_structs();
    an_example_program_using_structs();
    method_syntax();
}

fn method_syntax() {
    println!("== Method Syntax ==");
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    let square = Rectangle::square(2);
    println!("Square of 2 is {:?}", square);
    Rectangle::hello("hello, world!");
}


// method syntax: implementation of a method on the Rectangle struct
impl Rectangle {
    // Struct Method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // Struct Method
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // Associated function
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
// multiple imp blocks
impl Rectangle {
    fn hello(message: &str) {
        println!("{}", message);
    }
}

fn an_example_program_using_structs() {
    println!("== An example program using structs ==");
    let rectangle = Rectangle{
        width: 30, height: 50
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle)
    );
    println!("rect1 is {:?}", rectangle);
    println!("rect1 is {:#?}", rectangle);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn defining_and_instanciating_structs() {
    println!("== Defining and Instanciating Structs ==");
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1);
    user1.email = String::from("another@email.com");
    println!("{}", user1);

    let another_user = build_user(String::from("hello@world.com"), String::from("sniggel"));
    println!("{}", another_user);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user2);

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // use the rest of the fields of user2 to fill in the missing infos (nice syntaxic sugar)
        ..user2
    };
    println!("{}", user3);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Color: {} {} {}", black.0, black.1, black.2);
    println!("Point: {} {} {}", origin.0, origin.1, origin.2);
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        // Struct Init shorthand: field and values have the same names
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User:\n\tusername: {}\n\temail: {}\n\tsign_in_count: {}\n\tactive: {}", self.username, self.email, self.sign_in_count, self.active)
    }
}