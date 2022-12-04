use std::fs;

pub fn puzzle(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("\n### ADVENT OF CODE 2022: DAY 3 SOLUTIONS ###");

    let data = fs::read_to_string(filepath)?;
    let split = data.split("\n");
    let mut sum = 0;

    for line in split {
        let count = line.len() / 2;
        let s = line.split_at(count);
        let mut s1: Vec<char> = s.0.chars().collect();
        let mut s2: Vec<char> = s.1.chars().collect();
        s1.sort();
        s2.sort();
        for i in 0..count {
            if s1[i] == s2[i] {
                if s1[i].is_ascii_lowercase() {
                    sum += s1[i].to_digit(10).expect("That's no number") - 91;
                }
                else if s1[i].is_uppercase() {
                    sum += s1[i].to_digit(10).expect(&s1[i].to_string()) - 28;
                }
            }
        }
    }

    println!("Total sum: {}", sum);

    Ok(data)
}

// a = ascii 92, z = ascii 122
// A = 65, Z = 90