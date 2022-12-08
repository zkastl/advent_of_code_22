use std::fs;

pub fn puzzle(filepath: &str) {
    println!("\n### ADVENT OF CODE 2022: DAY X SOLUTIONS ###");

    let raw = fs::read_to_string(filepath).unwrap();
    let data: Vec<char> = raw.chars().collect();
    for i in 0..data.len()-3 {
        let buffer = &raw[i..i+4];
        println!("{}", buffer);
    }
}