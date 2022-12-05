use std::fs;

pub fn puzzle(filepath: &str) {
    println!("\n### ADVENT OF CODE 2022: DAY 5 SOLUTIONS ###");

    let mut counter = 0;
    for line in fs::read_to_string(filepath).expect("Help! Bad input data!").split("\n") {

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

    }
}