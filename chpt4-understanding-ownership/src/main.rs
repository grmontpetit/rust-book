use std::fmt;

fn main() {
    scoping();
    strings();
    cloning();
    copying();
    ownership();
    return_values_and_scope();
    returning_ownership();
    references();
}

fn references() {
    println!("== References ==");
    let s1 = String::from("hello");

    // here, calculate_length_with_ref is "borrowing" s1.
    let len = calculate_length_with_ref(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // mutable reference
    let mut s2 = String::from("hi");
    change(&mut s2);
    println!("{}", s2);

    // dangling reference vs. no dangling
    // let d = dangle();
    println!("no dangle: {}", no_dangle());
}

// Below will not compile
// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");
    // return the string, not the ref
    s
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// s is a reference to a String, so the function doesn't take ownership of it.
// This is called borrowing.
fn calculate_length_with_ref(s: &String) -> usize {
    // The reference is not mutable
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens

fn returning_ownership() {
    println!("== Returning Ownership ==");
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn return_values_and_scope() {
    println!("== Returning Values and Scope ==");
    let _s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn ownership() {
    println!("== Ownership ==");
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn copying() {
    // happens on the stack: faster
    println!("== Copying ==");
    // let x: i32 = 5;
    // let z = x.Copy();
    is_copy::<bool>();
    is_copy::<char>();
    is_copy::<i8>();
    is_copy::<i16>();
    let ic = is_copy::<i32>();
    println!("{}", ic);
    is_copy::<i64>();
    is_copy::<u8>();
    is_copy::<u16>();
    is_copy::<u32>();
    is_copy::<u64>();
    is_copy::<isize>();
    is_copy::<usize>();
    is_copy::<f32>();
    is_copy::<f64>();
    is_copy::<fn()>();
    // Mutable aren't copyable because they are stored in the heap (they have an unknown sizes)
    // is_copy::<&mut i32>();

    let point = Point { x: 1.0, y: 2.0 };

    let a_struct = Rectangle{p1: point, p2: Point{x: 9.0, y: 10.0}};
    println!("{}", a_struct);
    // doesn't derive Copy
    // is_copy(a_struct);
}

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle: (p1: ({}, {}) p2: ({}, {}))", self.p1.x, self.p1.y, self.p2.x, self.p2.y)
    }
}

fn is_copy<T: Copy>() -> bool { return true }

fn cloning() {
    // happens on the heap: slower
    println!("== Cloning ==");
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{} {}", s1, s2);
}

fn scoping() {
    println!("== Scoping ==");
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
        println!("{}", s);
    }
    // s not found (s is out of scope)
    // println!("{}", s);
}
fn strings() {
    println!("== Strings ==");
    // this is a string reference, pushed on the stack
    let some_string: &str = ", world";

    // this is a non-mutable string literal
    let a_string_literal = String::from(", world");

    // this is a mutable string literal, stored on the heap
    let mut a_mutable_string: std::string::String = String::from("hello");
    a_mutable_string.push_str(some_string);

    println!("{}", a_mutable_string);

    a_mutable_string.push_str(&a_string_literal);
    println!("{}", a_mutable_string);

    let s1 = String::from("hello");
    // s1 becomes invalid so that drop is not called twice when s2 goes out of scope
    // this action is called a move
    let _s2 = s1;

    // s1 is no longer valid because it was out of scoped when assigning s1 to s2.
    // println!("{}", s1);
}