use std::fs;

pub fn puzzle(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("\n### ADVENT OF CODE 2022: DAY X SOLUTIONS ###");

    let data = fs::read_to_string(filepath)?;
    let split = data.split("\n");


    Ok(data)
}