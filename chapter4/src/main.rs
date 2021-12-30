fn main() {
    // Ownership rules
    // 1. Each value in Rust has a variable that's called its owner.
    // 2, There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    { // s is not valid here, it's not declared yet
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s...
    } // this scope is now over, and s is no longer valid
}

// OWNERSHIP
// ---------------------------------------------------------------------------------

fn function1() {
    let x = 5;
    let y = x; // Copy

    let s1 = String:from("hello");
    let s2 = s1; // Moves s1 to s2, invalidating s1 (not shallow copy)

    // Rust defaults to moving a value
    // if we want to clone we need to use `.clone()` value
    let s1Clone = String:from("hello");
    let s2Clone = s1.clone
}


// OWNERSHIP TRANSFER
// ---------------------------------------------------------------------------------

fn taker() {
    let s = String::from("Hhello");
    takes_ownership(s) //  Moves `s`

    let x = 5;
    makes_copy(x) // Copies `x`
}

// Can take ownership + give it back too, just return `String`
fn takes_ownership(some_string: String) {}
fn makes_copy(some_integar: i32) {}


// REFERENCES
// ---------------------------------------------------------------------------------
// - references don't take ownership

fn ref_storage() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // pass s1 as reference, not moving
}

// takes in reference (&) of String as param (aka, borrowing)
fn calculate_length(s: &String) -> usize {
    // modifying `s` in this function will not work
    let length = s.len(); // len() returns the length of a String
    length;
}


// CHANGING VALUE W/O OWNERSHIP
// ---------------------------------------------------------------------------------
// - Can only borrow 1 mutable value at a time b/c there's no sync when reading + writing for 2 data points
// - Cannot borrow mutable reference when reference is also being used as immutable
// - Can have multiple immutable references b/c underlying data isn't going to change

fn ownership_storage() {
    let mut s1 = String::from("hello");
    // pass mutable reference
    change_value(&mut s1);
}

// takes in mutable reference
fn change_value(some_string: &String) {
    some_string.push_str(", world");
}


// RULES OF REFERENCES
// ---------------------------------------------------------------------------------
// 1. At any given time, you can have either one mutable reference or any number of immutable references.
// 2. References must also be valid.


// SLICES
// ---------------------------------------------------------------------------------
// STRING SLICES
fn word_storage {
    let mut s = String::from("hello world");

    let s2 = "hello world";

    let word = first_word(s2)

}

fn first_word(s: &str) -> &str {
    // take string, turn into array of bytes
    let bytes = s.as_bytes();

    // get index of each item + the item itself
    for (i, &item) in bytes.iter().enumerate() {
        // for every item we check for empty space that signifies end of word
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ARRAY SLICES
fn array_storage() {
    let a = [1, 2, 3, 4, 5];
    // slice references 0 - 2
    let slice = %a[0..2];
}