// Start of Selection
use serde_json::Value;
use std::fs;

fn ask_question(question: &Value, q_number: usize) {
    println!(
        "Question {}: {}",
        q_number,
        question["question"].as_str().unwrap()
    );
    for (i, option) in question["options"].as_array().unwrap().iter().enumerate() {
        println!("{}. {}", i + 1, option.as_str().unwrap());
    }
}

fn check_answer(question: &Value, answer: &str) -> bool {
    question["answer"].as_str().unwrap() == answer.trim()
}

pub fn quiz(player_name: String) {
    println!("Hello, {}! Let's play quiz!", player_name);
    let mut correct_answers = 0;
    let mut answered_questions = 0;
    let quiz_content = fs::read_to_string("quiz.json").expect("Failed to read quiz.json");
    let quiz: Value = serde_json::from_str(&quiz_content).expect("Failed to parse quiz.json");
    let total_questions = quiz["quiz"].as_array().unwrap().len();

    for (i, question) in quiz["quiz"].as_array().unwrap().iter().enumerate() {
        ask_question(question, i + 1);
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).unwrap();

        if answer.trim().eq_ignore_ascii_case("quit") {
            println!("You chose to quit the quiz.");
            break;
        }

        match check_answer(question, &answer) {
            true => {
                println!("Nice! You got it right!");
                correct_answers += 1;
            }
            false => {
                println!("Your answer: {}", answer.trim());
                println!(
                    "The correct answer is {}",
                    question["answer"].as_str().unwrap()
                );
                println!("Oops! You got it wrong!");
            }
        }
        answered_questions += 1;

        println!(
            "--- Question No - {} & Ques Left - {} ---",
            answered_questions + 1,
            total_questions - answered_questions
        );
    }
    let score = match answered_questions > 0 {
        true => (correct_answers as f32 / total_questions as f32) * 100.0,
        false => 0.0,
    };
    println!(
        "-------Quiz Summary-------\nTotal Questions: {}\nAnswered Questions: \t{}\nCorrect Answers: \t{}\nScore: \t{}%",
        total_questions, answered_questions, correct_answers, score
    );
}
