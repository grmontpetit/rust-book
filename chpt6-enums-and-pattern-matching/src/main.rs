
fn main() {
    enums();
    matching_option();
    concise_control_flow();
}

fn concise_control_flow() {
    println!("== Concise Control Flow ==");
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("Not three.");
    }
}

fn matching_option() {
    println!("== Matching Option ==");
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
struct Version8 {
    address: String
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
    V7(u8, u8, u8, u8),
    V8(Version8),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// #[derive(Debug)]
// struct QuitMessage; // unit struct
// #[derive(Debug)]
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// #[derive(Debug)]
// struct WriteMessage(String); // tuple struct
// #[derive(Debug)]
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
        match self {
            Message::Quit => println!("quitting"),
            Message::Move{x, y} => println!("moving to {} {}", x, y),
            Message::Write(s) => println!("writing {}", s),
            Message::ChangeColor(r, g, b) => println!("changing color to RGB: {} {} {}", r, g, b),
        }
    }
}

fn enums() {
    println!("== Enums ==");
    // concise version
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    let version_7 = IpAddr::V7(127, 0, 0, 1);
    let version_8 = IpAddr::V8(Version8{address: String::from("888.888.888.888")});
    route(home);
    route(loopback);
    route(version_7);
    route(version_8);

    let mut m = Message::Write(String::from("hello"));
    m.call();
    m = Message::Move{x: 1, y: 2};
    m.call();
    m = Message::ChangeColor(1, 2, 3);
    m.call();
    m = Message::Quit;
    m.call();
}

fn route(ip_kind: IpAddr) {
    println!("{:?}", ip_kind);
}