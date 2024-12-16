use rand::Rng;
use std::io;
fn main() {
    println!("Let's play guess the number!");

    let magic_number: u32 = rand::thread_rng().gen_range(1..=100);
    let mut name = String::new();
    println!("Please enter your name.");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Hello, {}! Let's play guess the number!", name);

    let mut guess_count: u32 = 0;
    let guess_limit: u32 = 10;
    let mut game_over = false;
    loop {
        println!("Please input your guess.\nYou have only {} guesses left.", guess_limit - guess_count);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if guess > 100 || guess < 1 {
            println!("Please enter a number between 1 and 100.");
            continue;
        }

        if guess < magic_number {
            println!("Too small!");
        } else if guess > magic_number {
            println!("Too big!");
        } else {
            println!("You win! {} is the secret number!", guess);
            break;
        }
        guess_count += 1;
        if guess_count >= guess_limit {
            game_over = true;
            break;
        }
    }

    if game_over {
        println!("Game over! You guessed {} times.\n and the secret number was {}", guess_count, magic_number);
    }
    println!("Thanks for playing! Your score is: {}",);

}
