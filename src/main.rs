use std::io;
mod guess_game;
mod quiz;

fn main() {
    let mut choice = String::new();
    println!("Please choose a game to play.");
    println!("1. Guess the number");
    println!("2. Quiz");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please enter a valid number");
    match choice {
        1 => guess_game::guess_game(),
        2 => quiz::quiz(),
        _ => println!("Invalid choice"),
    }
}
