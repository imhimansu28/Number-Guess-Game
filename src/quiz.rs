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

pub fn quiz() {
    let mut correct_answers = 0;
    let quiz_content = fs::read_to_string("quiz.json").expect("Failed to read quiz.json");
    let quiz: Value = serde_json::from_str(&quiz_content).expect("Failed to parse quiz.json");

    for (i, question) in quiz["quiz"].as_array().unwrap().iter().enumerate() {
        ask_question(question, i + 1);
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).unwrap();
        match check_answer(question, &answer) {
            true => {
                println!("Correct!");
                correct_answers += 1;
            }
            false => {
                println!("Your answer: {}", answer.trim());
                println!(
                    "The correct answer is {}",
                    question["answer"].as_str().unwrap()
                );
                println!("Incorrect!");
            }
        }
        println!("--------------------------------");
    }
    let total_questions: usize = quiz["quiz"].as_array().unwrap().len();
    let score = (correct_answers as f32 / total_questions as f32) * 100.0;
    let score_message = match score {
        _ if score >= 55.0 => "Hurray! You passed the quiz!".to_string(),
        _ => format!(
            "Better luck next time! You got {} out of {} correct!",
            correct_answers, total_questions
        ),
    };
    println!("{}", score_message);
}
