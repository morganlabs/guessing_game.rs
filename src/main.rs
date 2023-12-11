extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write; // <--- bring flush() into scope

fn main() {
    // Generate a Random Number
    let rand_num: u8 = rand::thread_rng().gen_range(1..=100);
    dbg!(rand_num);

    println!("Welcome to the Guessing Game!");

    let name = prompt("What's your name?");
    println!("\nWelcome to the Guessing Game, {}.", name);

    let mut attempts: u8 = 1;
    while attempts <= 3 {
        let ans = take_guess(rand_num, attempts);

        match ans.cmp(&0) {
            Ordering::Less => {
                if attempts != 3 {
                    println!("Too low! Try again.");
                };
            }
            Ordering::Greater => {
                if attempts != 3 {
                    println!("Too high! Try again.");
                };
            }
            Ordering::Equal => {
                println!("Correct! The number was {}", rand_num);
                return;
            }
        }
        attempts += 1;
    }
    println!("Too bad! The number was {}", rand_num);
}

fn take_guess(rand_num: u8, attempt: u8) -> i8 {
    let guess = prompt(format!("What number am I thinking of? (Attempt {}/3)", attempt).as_str());
    let guess: u8 = guess.parse().expect("Could not parse guess to type u32");

    match guess.cmp(&rand_num) {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 0,
    }
}

fn prompt(prompt: &str) -> String {
    print!("{}: ", prompt);

    // Flush the buffer
    io::stdout().flush().expect("Failed to flush the stdout.");

    // Get the answer
    let mut ans = String::new();
    io::stdin()
        .read_line(&mut ans)
        .expect("Failed to read line.");

    return ans.trim().to_string();
}
