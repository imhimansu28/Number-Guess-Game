# Start Generation Here

## How to Run the Guess Game

### Prerequisites

- **Rust and Cargo**: Ensure you have Rust and Cargo installed. You can install them from [rustup.rs](https://rustup.rs/).

### Steps to Run

1. **Clone the Repository**

   ```sh
   git clone https://github.com/yourusername/guess_game.git
   ```

2. **Navigate to the Project Directory**

   ```sh
   cd guess_game
   ```

3. **Build and Run the Application**

   ```sh
   cargo run
   ```

### How the Game Works

1. Upon running the application, you'll be prompted to enter your name.
2. The game will generate a random secret number between 1 and 100.
3. You have 10 attempts to guess the secret number.
4. After each guess, you'll receive feedback indicating whether your guess was too low or too high.
5. If you guess the number within 10 attempts, you win! Otherwise, the game ends, revealing the secret number.

### Example

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/guess_game`
Let's play guess the number!
Please enter your name.
Alice
Hello, Alice! Let's play guess the number!
You have 10 guesses left.
Please input your guess.
50
Too small!
You have 9 guesses left.
Please input your guess.
75
Too big!
...
```

# End Generation Here

```

```
