use std::fs;

pub fn puzzle(filepath: &str) {
    println!("\n### ADVENT OF CODE 2022: DAY 4 SOLUTIONS ###");

    let mut sum = 0;
    let mut sum2 = 0;

    for line in fs::read_to_string(filepath)
    .expect("Help! Bad input data!").split("\n") {
        let data: Vec<&str> = line.split_terminator(',').collect();
        if data.len() != 2 {
            panic!("VECTOR LENGTH INVALID");
        }

        let d1: Vec<&str> = data[0].split_terminator('-').collect();
        let pair1 = 
            (d1[0].parse::<i32>().unwrap(), d1[1].parse::<i32>().unwrap());
        
        let d2: Vec<&str> = data[1].split_terminator('-').collect();
        let pair2 = 
            (d2[0].parse::<i32>().unwrap(), d2[1].parse::<i32>().unwrap());

        if (pair1.0 <= pair2.0 && pair1.1 >= pair2.1) ||
             (pair2.0 <= pair1.0 && pair2.1 >= pair1.1) {
            println!("Complete overlap found: {}", line);
            sum += 1;
        }

        if !((pair1.0 < pair2.0 && pair1.1 < pair2.0) ||
             (pair1.0 > pair2.1 && pair1.1 > pair2.1)) {
            println!("Partial overlap found: {}", line);
            sum2 +=1;
        }
    }

    println!("NUMBER OF COMPLETELY ENCLOSED PAIRS: {}", sum);
    println!("NUMBER OF PARTIALLY ENCLOSED PAIRS: {}", sum2);
}