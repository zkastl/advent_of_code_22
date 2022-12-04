use std::fs;

pub fn puzzle(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("\n### ADVENT OF CODE 2022: DAY 3 SOLUTIONS ###");

    let data = fs::read_to_string(filepath)?;
    let mut sum = 0;
    let mut sum2 = 0;

    for line in data.split("\n") {
        let count = line.len() / 2;
        let s = line.split_at(count);
        sum += find_priority(match_char(s.0, s.1));
    }
    println!("Total sum: {}", sum);

    let split_data: Vec<&str> = data.split("\n").collect();
    let mut l: Vec<&str> = Vec::new();
    for value in split_data {
        l.push(value);
        if l.len() == 3 {
            sum2 += find_priority(match_char2(l[0], l[1], l[2]));
            l.clear();
        }
    }
    println!("Total sum: {}", sum2);

    Ok(data)
}

fn match_char(str1: &str, str2: &str) -> char {
    let s1: Vec<char> = str1.chars().collect();
    let s2: Vec<char> = str2.chars().collect();
    let count = str1.len();
    for i in 0..count {
        for j in 0..count {
            if s1[i].eq(&s2[j]) {
                return s1[i];
            }
        }
    }
    return '0';
}

fn match_char2(str1: &str, str2: &str, str3: &str) -> char {
    let s1: Vec<char> = str1.chars().collect();
    let s2: Vec<char> = str2.chars().collect();
    let s3: Vec<char> = str3.chars().collect();

    for i in 0..s1.len() {
        for j in 0..s2.len() {
            for k in 0..s3.len() {
                if (s1[i] == s2[j]) && (s1[i] == s3[k]) {
                    println!("Found char: {}", s1[i]);
                    return s1[i];
                }
            }
        }
    }

    return '0';
}

fn find_priority(val: char) -> usize {
    let ref_string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let priority = ref_string.find(val);
    match priority {
        Some(v) => return v + 1,
        None => return 0,
    }
    
}

// a = ascii 92, z = ascii 122
// A = 65, Z = 90