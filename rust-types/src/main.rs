use std::io;

fn main() {
    // Integers
    // Signed (i prefix folllowed by byte representation with size matching the
    // system arch):
    //   - i8
    //   - i16
    //   - i32
    //   - i64
    //   - i128
    //   - isize
    // Unsigned (u prefix folllowed by byte representation with size matching
    // the system arch):
    //   - u8
    //   - u16
    //   - u32
    //   - u64
    //   - u128
    //   - usize

    // FLOATING POINT
    // Always signed and either f32 or f64 with f64 being preferred

    // Arithmetic:
    // addition
    let _sum = 10 + 5;

    // subtraciton
    let _diff = 10 - 5;

    // multiplication
    let _product = 10 * 5;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1
    let _remainder = 43 % 5;

    // Cannot mix types
    //let _product = _product * 10.0;
    //error[E0277]: cannot multiply `{integer}` by `{float}`
    //  --> types/src/main.rs:36:29
    //   |
    //36 |     let _product = _product * 10.0;
    //   |                             ^ no implementation for `{integer} *
    // {float}`   |
    //   = help: the trait `Mul<{float}>` is not implemented for `{integer}`
    //   = help: the following other types implement trait `Mul<Rhs>`:
    //             `&f128` implements `Mul<f128>`
    //             `&f128` implements `Mul`
    //             `&f16` implements `Mul<f16>`
    //             `&f16` implements `Mul`
    //             `&f32` implements `Mul<f32>`
    //             `&f32` implements `Mul`
    //             `&f64` implements `Mul<f64>`
    //             `&f64` implements `Mul`
    //           and 57 others

    //error: aborting due to 1 previous error

    // BOOLEANS
    // true or false and 1 byte in size
    let _t = true;
    let _f: bool = false;

    // CHARACTER TYPE
    // a “character” isn’t really a concept in Unicode, so your human intuition
    // for what a “character” is may not match up with what a char is in
    // Rust 4 byte size
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // COMPOUND TYPES: tuples/arrays
    // TUPLES
    // Tuples have a fixed length: Once declared, they cannot grow or shrink in
    // size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Can access by de-structuring
    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");
    // We can also access a tuple element directly by using a period (.)
    // followed by the index of the value we want to access.
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;
    // can also set the values by index
    let mut _x: (i32, i32) = (1, 2);
    _x.0 = 0;
    _x.1 += 5;

    // ARRAYS
    // Arrays have a fixed length and must contain only one type
    // implicit type
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // explicit type/size
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // can initialize an array given a length and value
    let _zs = ['z'; 5];

    // access via index
    let _first = a[0];
    let _second = a[1];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    // if out of bounds is accessed:
    // thread 'main' (844303) panicked at types/src/main.rs:132:19:
    // index out of bounds: the len is 5 but the index is 6
    // note: run with `RUST_BACKTRACE=1` environment variable to display a
    // backtrace
}
