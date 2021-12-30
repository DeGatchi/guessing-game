use colored::*;
use rand::Rng; // random number generator
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    // generate random number from 1 - 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // trim removes white space at end
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // no matter what error we get, continue to next iteration
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // compare `guess` to `secret_number` + return all return values
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You Win!".green());
                break;
            }
        }
    }
}
