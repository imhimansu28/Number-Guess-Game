use std::io;
mod guess_game;
mod quiz;

fn play_game(player_name: String, choice: u32) {
    match choice {
        1 => guess_game::guess_game(player_name),
        2 => quiz::quiz(player_name),
        _ => println!("Invalid choice"),
    }
}

fn main() {
    let mut player_name = String::new();
    println!("Please enter your name.");
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line");
    let mut choice = String::new();
    println!("Hello, {}! Please choose a game to play.", player_name);
    println!("1. Guess the number");
    println!("2. Quiz");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please enter a valid number");
    play_game(player_name, choice);
}
