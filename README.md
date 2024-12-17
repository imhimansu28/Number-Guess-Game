# Let's Play Some Games!

## How to Run the Games

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

### Available Games

1. **Guess the Number**
2. **Quiz**

### How the Games Work

#### Guess the Number

1. Upon selecting the "Guess the Number" game, you'll be prompted to enter your name.
2. The game will generate a random secret number between 1 and 100.
3. You have 10 attempts to guess the secret number.
4. After each guess, you'll receive feedback indicating whether your guess was too low or too high.
5. If you guess the number within 10 attempts, you win! Otherwise, the game ends, revealing the secret number.

#### Quiz

1. Upon selecting the "Quiz" game, you'll be presented with a series of multiple-choice questions.
2. Each question has four options; enter the number corresponding to your choice.
3. After each answer, you'll receive feedback indicating whether your answer was correct.
4. At the end of the quiz, your total score will be displayed, showing how many questions you answered correctly.

### Example

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/guess_game`
Please choose a game to play.
1. Guess the Number
2. Quiz
```
