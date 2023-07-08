use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Define the score for each choice
    let mut choice_score = HashMap::new();
    choice_score.insert("X", 1);
    choice_score.insert("Y", 2);
    choice_score.insert("Z", 3);

    // Define the score for each outcome
    let mut outcome_score = HashMap::new();
    outcome_score.insert("X", 0); // Loss
    outcome_score.insert("Y", 3); // Draw
    outcome_score.insert("Z", 6); // Win

    // Define your choice based on the opponent's choice and the outcome
    let mut outcome_choice = HashMap::new();
    outcome_choice.insert(("A", "X"), "Z"); // If opponent chose Rock and you lost, you chose Scissors
    outcome_choice.insert(("B", "X"), "X"); // If opponent chose Paper and you lost, you chose Rock
    outcome_choice.insert(("C", "X"), "Y"); // If opponent chose Scissors and you lost, you chose Paper
    outcome_choice.insert(("A", "Y"), "X"); // If opponent chose Rock and you drew, you chose Rock
    outcome_choice.insert(("B", "Y"), "Y"); // If opponent chose Paper and you drew, you chose Paper
    outcome_choice.insert(("C", "Y"), "Z"); // If opponent chose Scissors and you drew, you chose Scissors
    outcome_choice.insert(("A", "Z"), "Y"); // If opponent chose Rock and you won, you chose Paper
    outcome_choice.insert(("B", "Z"), "Z"); // If opponent chose Paper and you won, you chose Scissors
    outcome_choice.insert(("C", "Z"), "X"); // If opponent chose Scissors and you won, you chose Rock

    // Open the file
    let path = Path::new("./input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Calculate the total score
    let mut total_score = 0;
    for line in reader.lines() {
        let line = line?;
        let choices: Vec<&str> = line.split_whitespace().collect();
        let my_choice = outcome_choice[&(choices[0], choices[1])];
        total_score += choice_score[my_choice];
        total_score += outcome_score[choices[1]];
    }

    println!("{}", total_score);

    Ok(())
}