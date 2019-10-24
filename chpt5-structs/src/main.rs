use std::fmt;

fn main() {
    defining_and_instanciating_structs();
}

fn defining_and_instanciating_structs() {
    println!("== Defining and Instanciating Structs ==");
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1);
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