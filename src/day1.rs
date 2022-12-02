use std::fs;

pub fn puzzle(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("### ADVENT OF CODE 2022: DAY 1 SOLUTIONS ###");

    let data = fs::read_to_string(filepath)?;
    let split = data.split("\n");
    let mut elves = Vec::new();
    elves.push(0);

    for s in split {
        let last_pos = elves.len() - 1;
        if s.is_empty() {
            elves.push(0)
        }
        else {
            elves[last_pos] += s.parse::<i32>().unwrap();
        }
    };

    elves.sort();
    match elves.iter().max() {
        Some(max) => 
            println!("Max Calories: {}", max),
        None => 
            println!("empty"),
    }

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