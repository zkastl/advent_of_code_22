use std::fs;

pub fn puzzle(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("\n### ADVENT OF CODE 2022: DAY 3 SOLUTIONS ###");

    let data = fs::read_to_string(filepath)?;
    let split = data.split("\n");
    let mut sum = 0;

    for line in split {
        let ref_string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let count = line.len() / 2;
        let s = line.split_at(count);
        let mut s1: Vec<char> = s.0.chars().collect();
        let mut s2: Vec<char> = s.1.chars().collect();
        for i in 0..count {
            for j in 0..count {
                if s1[i].eq(&s2[j]) {
                    let val = ref_string.find(s1[i]).unwrap() + 1;
                    println!("Found match: {}, value: {}", s1[i], val);
                    sum += val;
                    break;
                }
            }
            
        }
    }

    println!("Total sum: {}", sum);

    Ok(data)
}

fn match_char(str1: &str, str2: &str) -> char {
    return 'a';
}

// a = ascii 92, z = ascii 122
// A = 65, Z = 90