use std::fs;

pub fn puzzle(filepath: &str) {
    println!("\n### ADVENT OF CODE 2022: DAY 6 SOLUTIONS ###");

    let raw = fs::read_to_string(filepath).unwrap();
    let data: Vec<char> = raw.chars().collect();
    
    for i in 0..data.len()-3 {
        let buffer = &raw[i..i+4];
        
        println!("Signal buffer: {}", buffer);

        if !is_unique(buffer) {
            continue;
        }

        println!("Signal start at {}", i+4);
        break;
    }

    for j in 0..data.len()-3 {
        let buffer2 = &raw[j..j+14];
        
        println!("Message buffer: {}", buffer2);

        if !is_unique(buffer2) {
            continue;
        }

        println!("Message start at {}", j+14);
        break;
    }
}

fn is_unique(test_str: &str) -> bool {

    if test_str.len() == 0 {
        return false;
    }
    
    // Create a table of 'used' chars.
    let mut used_chars: Vec::<char> = Vec::new();
    let char_table: Vec::<char> = test_str.chars().collect();

    // Break up string and loop through
    for i in 0..test_str.len() {
        if used_chars.contains(&char_table[i]) {
            return false;
        }
        else {
            used_chars.push(char_table[i]);
        }
    }

    return true;
}