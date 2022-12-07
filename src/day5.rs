use std::fs;

pub fn puzzle(filepath: &str) {
    println!("\n### ADVENT OF CODE 2022: DAY 5 SOLUTIONS ###");

    let max_stacks = 9;
    let mut _counter = 0;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut data: Vec<&str> = Vec::new();
    for _i in 0..max_stacks {
        stacks.push(Vec::<char>::new());
    }
    for line in fs::read_to_string(filepath).expect("Help! Bad input data!").split("\n") {
        if line.is_empty() {
            // format data
            data.pop();
            while !data.is_empty() {                
                let total_char = 2 * max_stacks - 1;
                let filtered_line: Vec<char> = data.pop().expect("INVALID STRING").chars().filter(|x| x != &'[' && x != &']').collect();
                assert!(filtered_line.len() == total_char, "{}", line);

                for i in 0..filtered_line.len() {
                    if i % 2 == 0 && filtered_line[i] != ' ' {
                        let pos = i/2;
                        stacks[pos].push(filtered_line[i]);
                    }
                }
            }
        }

        else {
            // queue data
            if !line.contains("move") {
                data.push(line);
                continue;
            }

            let command: Vec<&str> = line.split(" ").collect();
            assert!(command.len() == 6, "INVALID COMMAND LENGTH");
            println!("Current line: {}", line);
            let x = command[1].parse::<usize>().unwrap();
            let y = command[3].parse::<usize>().unwrap();
            let z = command[5].parse::<usize>().unwrap();
            let mut val1: Vec::<char> = Vec::new();

            for _i in 0..x {
                val1.push(stacks[y-1].pop().unwrap());
            }

            val1.reverse(); // comment out for part 1
            stacks[z-1].append(&mut val1);

        }

        // Basic plan:
            // Read the source data into two sections. Part 1 / Part 2 / 3
                    
        // Part 1:
            // Read until the line ' 1  2  3 ...' is read. This marks the final line of section 1,
            // format this data into a vector of vectors (a vector of stacks)
            // This will allow each stack to have the top crate as the last element for 'pop' purposes
                    
        
        // part 2:
            // Process each line as an action of the crane, e.g 'move 1 from 1 to 2' would mean:
                // from stack 1, for each in range of 1, pop from stack 1 and push on stack 2

        // part 3:
            // when part 2 is finished, read the top element of each stack 'blanks do nothing' and concat, return

    // part 3    

    }

    while stacks.len() > 0 {
        let s = stacks.pop();
        for c in s.expect("") {
            print!("{}",c);
        }
        println!("");
    }
}