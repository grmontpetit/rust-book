
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
    println!("non-mutable: {}\n mutable: {}\n constant: {}", x, y, MAX);

    /* Shadowing */
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}

fn data_types() {
    /* Scalar: represents a single value */

    // Integer Types (signed and unsigned)
    let a: i8 = 1;
    let b: u16 = 1;
    let c: i32 = 1;
    let d: u64 = 1;
    let e: i128 = 1;
    let f: usize = 1;
    let g: isize = 1;
    println!("{} {} {} {} {} {} {}", a, b, c, d, e, f, g)
    // watch-out for integer overflow when assigning a value to a variable outside of its range.



    /* Compound */

}