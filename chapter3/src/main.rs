fn main() {
    // variables are immutable by default
    // mut makes them mutable
    let mut x = 5;
    x = 6;

    // constants w/ never be mutated + are type annotated
    const SUBSCRIBER_COUNT: u32 = 100_000;

    // SHADOWING
    let y = 5;
    let y = "six";

    // DATA TYPES

    // Integers
    let a = 98_222; // decimal
    let b = 0xff; // hex
    let c = 0o77; // octal
    let d = 0b1111_0000; // binary
    let e = b'A'; // byte (u8 only)

    // Compound types
    let tup = ("let's get rusty", 100_000);
    let (channel , sub_count) = tup;
    let sub_count = tup.1;

    let error_codes = [200, 404];
    let not_found = error_codes[1];
    let x = error_codes[3]; // out of bounds (memory unsafe)

    let byte = [0; 8]; // create 8 elements w/ values of 0

    my_function(11, 22);

    // CONTROL FLOW

    let number = 5;

    if number < 10 {
        
    } else if number < 22 {

    } else 

    let condition = true;
    let nomber = if condition { 5 } else { 6 };
}

// FUNCTIONS (fn)

// snakecase - all lowercase + spaces with `_`
fn my_function(x: i32, v: i32) -> i32 {
    let sum = x + y; // expression
    // return sum;
    // or 
    // x + y 
}

fn looper() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    // while number !== 0 {
    //     // print
    // }

    // let a = [10, 20, 30];

    // for every element in `a`
    // for element in a.iter() {
    //     // print
    // }

    // for every number in this range, print it out
    // for number in 1..4 {
        // print
    // }
}