fn main() {
    if_conditionals();

    // Can use the conditional to assign a variable
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // must assign values of the same type
    //let number = if condition { 5 } else { "six" };
    //println!("The value of number is: {number}");
    //error[E0308]: `if` and `else` have incompatible types
    //  --> rust-control-flow/src/main.rs:10:44
    //   |
    //10 |     let number = if condition { 5 } else { "six" };
    //   |                                 -          ^^^^^ expected integer,
    // found `&str`   |                                 |
    //   |                                 expected because of this

    //error: aborting due to 1 previous error

    loop_fn();
    loop_label();
    while_loop();
    iterating_collection();
    countdown_with_for();
    let x = (1..4).rev();
    println!("x is: {:?}", x);
}

// Rust will not automatically try to convert non-Boolean types to a Boolean
// executes the first body for which the condition evaluates to true
fn if_conditionals() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else if number == 3 {
        println!("who-cares");
    } else {
        println!("condition was false");
    }
}

// LOOPS: loop, while, for
// loop type indefinitely executes an action until terminated
fn loop_fn() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // break executes as a scoped return setitng counter to 20
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// loops can be labeled to control the break context when loops are in loops
fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn iterating_collection() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn countdown_with_for() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
