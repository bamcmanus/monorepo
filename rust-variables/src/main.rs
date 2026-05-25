fn main() {
    let x = 5;
    println!("The value of x is {x}");
    //x = 6;
    //println!("The value of x is {x}");
    // error[E0384]: cannot assign twice to immutable variable `x`
    //  --> variables-rust/src/main.rs:4:5
    //   |
    // 2 |     let x = 5;
    //   |         - first assignment to `x`
    // 3 |     println!("The value of x is {x}");
    // 4 |     x = 6;
    //   |     ^^^^^ cannot assign twice to immutable variable
    //   |
    // help: consider making this binding mutable
    //   |
    // 2 |     let mut x = 5;
    //   |         +++

    // error: aborting due to 1 previous error
    let mut y = 2;
    println!("The value of y is {y}");
    y = 3;
    println!("The value of y is {y}");
    // No Error

    // CONSTANTS
    // needs type
    const Z: u32 = 42;
    println!("The value of z is {Z}");
    //z = 12;
    //println!("The value of z is {z}");
    //error[E0070]: invalid left-hand side of assignment
    //  --> variables-rust/src/main.rs:29:7
    //   |
    //29 |     z = 12;
    //   |     - ^
    //   |     |
    //   |     cannot assign to this expression

    //error: aborting due to 1 previous error

    // SHADOWING
    let x = 5;
    // shadows above
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // TYPE MANIPULATION
    //let mut spaces = "   ";
    //spaces = spaces.len();
    //error[E0308]: mismatched types
    //  --> variables-rust/src/main.rs:57:14
    //   |
    //56 |     let mut spaces = "   ";
    //   |                      ----- expected due to this value
    //57 |     spaces = spaces.len();
    //   |              ^^^^^^^^^^^^ expected `&str`, found `usize`

    //error: aborting due to 1 previous error
    // BUT....
    let spaces = "   ";
    let spaces = spaces.len();
    println!("This is spaces: {spaces}")
}
