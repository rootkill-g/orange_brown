use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::prelude::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Please input your guess : ");

        // Since print macro does not flush stdout automatically like println macro,
        // so we have to manually flush stdout after using the print macro
        io::stdout().flush().ok().expect("Failed to flush stdout");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("You guessed : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
        }
    }
}
