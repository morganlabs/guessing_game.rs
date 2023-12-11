use std::io;
use std::io::Write; // <--- bring flush() into scope

fn main() {
    println!("Welcome to the Guessing Game!");

    let name = prompt("What's your name?");
    println!("Welcome to the Guessing Game, {}.", name);
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
