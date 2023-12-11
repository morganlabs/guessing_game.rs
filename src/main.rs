extern crate rand;

use rand::Rng;
use std::io;
use std::io::Write; // <--- bring flush() into scope

fn main() {
    // Generate a Random Number
    let rand_num = rand::thread_rng().gen_range(1..=100);
    dbg!(rand_num);

    println!("Welcome to the Guessing Game!");

    let name = prompt("What's your name?");
    println!("\nWelcome to the Guessing Game, {}.", name);

    let guess = prompt("What number am I thinking of?");
    dbg!(guess);
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
