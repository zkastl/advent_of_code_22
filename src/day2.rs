use std::fs;

pub fn puzzle(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("\n### ADVENT OF CODE 2022: DAY 2 SOLUTIONS ###");

    let data = fs::read_to_string(filepath)?;
    let split = data.split("\n");
    let mut score = 0;
    let mut score2 = 0;

    for s in split {
        match s {
            "A X" => {
                score += 4;
                score2 += 3;
            },
            "A Y" => {
                score += 8;
                score2 += 4;
            },
            "A Z" => {
                score += 3;
                score2 += 8;
            },
            "B X" => {
                score += 1;
                score2 += 1;
            },
            "B Y" => {
                score += 5;
                score2 += 5;
            },
            "B Z" => {
                score += 9;
                score2 += 9;
            },
            "C X" => {
                score += 7;
                score2 += 2;
            },
            "C Y" => {
                score += 2;
                score2 += 6;
            },
            "C Z" => {
                score += 6;
                score2 += 7;
            },
            &_ => println!("Error"),
        }
    }

    println!("Total score: {}", score);
    println!("Total score 2: {}", score2);
    Ok(data)
}