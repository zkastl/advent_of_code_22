use std::fs;

fn main() {
    println!("Hello, world!");

    day1("C:\\Users\\zkast\\source\\repos\\advent_of_code_22\\day1.txt").expect("help!");
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

    for s in split {
        
    }


    Ok(data)
}