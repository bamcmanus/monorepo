// Functions use fn keyword for declaration and snake_case by convention
fn main() {
    println!("Hello, world!");
    another_function(2);
    print_labeled_measurement(5, 'h');
    println!("The value of x is: {}", five());
    println!("The value of x is: {}", five_with_colon());
}

// function with params must have types
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// multiple params
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// return values are typed
// You can return early from a function by using the return keyword and
// specifying a value, but most functions return the last expression implicitly
// not the lack of ;
fn five() -> i32 {
    5
}
// when using semi-colon use explicit return
fn five_with_colon() -> i32 {
    return 5;
}
