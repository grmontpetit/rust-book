
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

    /* Functions */
    function_parameters(5);
    function_return_value();
    function_parameter_and_return_value();

    /* Comments */
    // tldr: double slash, slash star

    /* Control Flow */
    if_expressions();
    multiple_conditions();
    conditional_assignation();
    loop_return_values();
    while_loops();
    collection_looping();
    lift_off_v2();

    /* Exercices */
    temperature_converter();
    fibonacci();
    christmas_carol_lyrics();
}

fn christmas_carol_lyrics() {
    // TODO
}

fn fibonacci() {
    // TODO
}

fn temperature_converter() {
    // TODO
}

fn lift_off_v2() {
    println!("== Lift-Off: Version 2 ==");
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn collection_looping() {
    println!("== Looping Through a Collection: For ==");
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn while_loops() {
    println!("== While Loops ==");

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_return_values() {
    println!("== Return Values from Loops ==");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop result: {}", result);
}

fn conditional_assignation() {
    println!("== Conditional Assignations ==");

    let condition = true;

    let number = if condition {
        // type is i32
        5
    } else {
        // type is i32
        6
    // Notice the semi-colon below
    };
    println!("the conditional assignation is {}", number);
}

fn multiple_conditions() {
    println!("== If Expressions ==");
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
}

fn if_expressions() {
    println!("== If Expressions ==");
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
fn function_parameter_and_return_value() {
    println!("== Function Return Value And Parameter ==");
    let z = plus_one(5);
    println!("the value of z is {}", z);
}

fn plus_one(i: i32) -> i32 {
    return i + 1;
}

fn function_return_value() {
    println!("== Function Return Value ==");
    let x = five();
    println!("the value of x is {}", x);
}

fn five() -> i32 {
    return 5;
}

fn function_parameters(x: i32) {
    println!("== Function Paramters == ");
    println!("the value of x is {}", x);
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