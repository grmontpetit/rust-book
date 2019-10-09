
fn main() {
    variables_and_mutability();
    data_types();
}

fn variables_and_mutability() {
    // non-mutable variable
    let x: u32 = 1;
    // mutable variable
    let mut y: u32 = 2;
    y = y + 1;
    // constant
    const MAX: u32 = 1;
    println!("non-mutable: {} \nmutable: {}\nconstant: {}", x, y, MAX);

    /* Shadowing */
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}\n", x);
}

fn data_types() {
    /* Scalar: represents a single value */
    integer_types();
    floating_point_types();
    numeric_operations();
    boolean_type();
    character_type();

    /* Compound */
    tuple_type();
    array_type();

}

fn array_type() {
    println!("== Array Type ==");
    // arrays have fixed length
    // useful when you want to allocate on the stack
    // arrays aren't as flexible as vectors
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let strings = a.into_iter().map(|i| i.to_string() + ", ").collect::<String>();
    println!("this is an array: {}", strings);

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    match months.last()  {
        Some(x) => println!("the last month of the year is {}", x),
        None => println!("unknown month"),
    }

    match months.first() {
        Some(x) => println!("the first motn of the year is {}", x),
        None => println!("unknown month"),
    }
}

fn tuple_type() {
    println!("== Tuple Type ==");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring
    let (a, c, b) = tup;
    println!("this is a tuple: ({}, {}, {})", a, b, c);
}

fn character_type() {
    println!("== Character Type ==");
    let c = 'z';
    println!("c is {}", c);

    let z = 'ℤ';
    println!("z is {}", z);

    let heart_eyed_cat = '😻';
    println!("heart eyed cat is: {}", heart_eyed_cat);
}

fn boolean_type() {
    println!("== Boolean Types ==");
    let t = true;
    println!("t is {}", t);

    let f: bool = false;
    println!("f is {}", f);
}

fn numeric_operations() {
    println!("== Numeric Operations ==");
    let sum = 5 + 6;
    println!("5 + 6 = {}", sum);

    let difference = 95.5 - 60 as f32;
    println!("95.5 - 60 =  {}", difference);

    let product = 4 * 31;
    println!("4 * 31 = {}", product);

    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {}", quotient);

    let remainder = 45 % 5;
    println!("45 % 5 = {}", remainder);
}

fn floating_point_types() {
    println!("== Floating Point Types ==");
    // Floating-Point Types
    let float64 = 2.0; // f64
    let float32: f32 = 3.0;
    println!("this is a float64: {}\nthis is a float32: {}", float64, float32);

}

fn integer_types() {
    println!("== Integer Types ==");
    // Integer Types (signed and unsigned)
    let a: i8 = 1;
    let b: u16 = 1;
    let c: i32 = 1;
    let d: u64 = 1;
    let e: i128 = 1;
    let f: usize = 1;
    let g: isize = 1;
    println!("this is a i8: {}\nthis is a u16{}\nthis is a i32{}\nthis is a u64{}\nthis is a i128{}\nthis is a usize{}\nthis is a isize{}\n", a, b, c, d, e, f, g);
    // watch-out for integer overflow when assigning a value to a variable outside of its range.

    // Integer Literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    println!("this is a decimal: {}\nthis is a hex: {}\nthis is a octal: {}\nthis is a binary: {}\nthis is a byte: {}\n", decimal, hex, octal, binary, byte);
}