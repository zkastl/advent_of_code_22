use std::fs;

fn main() {
    println!("Hello, world!");

    //day1("C:\\Users\\zkast\\source\\repos\\advent_of_code_22\\day1.txt").expect("help!");
    day2("C:\\Users\\zkast\\source\\repos\\advent_of_code_22\\day2.txt").expect("help!");
    
}

fn day1(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    //println!("{}",data);

    let split = data.split("\n");
    let mut elves = Vec::new();
    elves.push(0);
    for s in split {
        let pos = elves.len() - 1;
        if s == "" {
            println!("{}", elves[pos]);
            elves.push(0)
        }
        else {
            elves[pos] += s.parse::<i32>().unwrap();
        }
    };

    let max_val = elves.iter().max();
    match max_val {
        Some(max) => println!("Max Calories: {}", max),
        None    => println!("empty"),
    }

    elves.sort();
    let mut sum = 0;
    for _i in 0..3 {
        let val = elves.pop();
        match val {
            Some(x) => sum += x,
            None => println!("error"),
        }
    }

    println!("Max of top three elves: {}", sum);

    Ok(data)
}

fn day2(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    let split = data.split("\n");
    //let mut elves = Vec::new();
    let mut score = 0;
    let mut score2 = 0;

    for s in split {
        print!("{}: ", s);
        match s {
            "A X" => { 
                println!("Rock/Rock: 4 points");
                println!("Rock/Lose(scissors): 3 points");
                score += 4;
                score2 += 3;
            },
            "A Y" => {
                println!("Rock/Paper: 8 Points");
                println!("Rock/Draw(rock): 4 Points");
                score += 8;
                score2 += 4;
            },
            "A Z" => {
                println!("Rock/Scissors: 3 Points");
                println!("Rock/Win(paper): 8 points");
                score += 3;
                score2 += 8;
            },
            "B X" => {
                println!("Paper/Rock: 1 Point");
                println!("Paper/Lose(rock): 1 Point");
                score += 1;
                score2 += 1;
            },
            "B Y" => {
                println!("Paper/Paper: 5 Points");
                println!("Paper/Draw(paper): 5 Points");
                score += 5;
                score2 += 5;
            },
            "B Z" => {
                println!("Paper/Scissors: 9 Points");
                println!("Paper/win(scissors): 9 Points");
                score += 9;
                score2 += 9;
            },
            "C X" => {
                println!("Scissors/Rock: 7 Points");
                println!("Scissors/Lose(paper): 2 Points");
                score += 7;
                score2 += 2;
            },
            "C Y" => {
                println!("Scissors/Paper: 2 Points");
                println!("Scissors/Draw(scissors): 6 Points");
                score += 2;
                score2 += 6;
            },
            "C Z" => {
                println!("Scissors/Scissors: 6 points");
                println!("Scissors/Win(rock): 7 points");
                score += 6;
                score2 += 7;
            },
            &_ => println!("Error"),
        }
    }

    println!("\nTotal score: {}", score);
    println!("Total score 2: {}", score2);
    Ok(data)
}