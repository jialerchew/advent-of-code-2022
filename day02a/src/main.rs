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

    // Define the outcome of each possible duel
    let mut duel_outcome = HashMap::new();
    duel_outcome.insert(("A", "X"), 3); // Draw
    duel_outcome.insert(("B", "Y"), 3); // Draw
    duel_outcome.insert(("C", "Z"), 3); // Draw
    duel_outcome.insert(("C", "X"), 6); // X beats C
    duel_outcome.insert(("A", "Y"), 6); // Y beats A
    duel_outcome.insert(("B", "Z"), 6); // Z beats B
    duel_outcome.insert(("B", "X"), 0); // X loses to B
    duel_outcome.insert(("C", "Y"), 0); // Y loses to C
    duel_outcome.insert(("A", "Z"), 0); // Z loses to A

    // Open the file
    let path = Path::new("./input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Calculate the total score
    let mut total_score = 0;
    for line in reader.lines() {
        let line = line?;
        let choices: Vec<&str> = line.split_whitespace().collect();
        total_score += choice_score[choices[1]];
        total_score += duel_outcome[&(choices[0], choices[1])];
    }

    println!("{}", total_score);

    Ok(())
}
