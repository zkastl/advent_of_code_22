use std::fs;

pub fn puzzle(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("\n### ADVENT OF CODE 2022: DAY 7 SOLUTIONS ###");

    let data = fs::read_to_string(filepath)?;
    let split: Vec::<&str> = data.split("\n").collect();

    for line in 0..split.len() {
        let cur_line = split[line];
        let tokens: Vec::<&str> = cur_line.split(" ").collect();
        assert!(tokens.len() > 0);
        match tokens[0] {
            "$" => println!("Command found: {}", tokens[1]),

            _ => println!("Data found: {}", cur_line),
        }
    }

    Ok(data)
}

fn map_filetree(name: String, history: Vec::<&str>, mut directory: Directory) -> Directory {

    let mut local_map: Directory = Directory { name: name, files: Vec::new(), subdirectories: Vec::new() };
    
    // Reach each line
    for line in 0..history.len() {
        let tokens: Vec::<&str> = history[line].split(" ").collect();
        match tokens[0] {
            "$" => {
                match tokens[1] {
                    "cd" => {
                        for dir in &directory.subdirectories {
                            
                    }
                    _ => println!("UNSUPPORTED COMMAND")
                }
            }

            "dir" => {
                let new_dir = Directory {
                    name: String::from(tokens[1]),
                    files: Vec::<File>::new(),
                    subdirectories: Vec::<Directory>::new(),
                };
                directory.subdirectories.push(new_dir);
            }

             _ => { }
        }
    }
    // is line a command?
    // yes: start new loop, read lines until you reach a new command
        // follow key words: 'cd' 'ls' 
    // no: allocate files into the tree

    //return the director
    Directory::default()
}

struct Directory {
    name: String,
    files: Vec::<File>,
    subdirectories: Vec::<Directory>,
}

impl Default for Directory {
    fn default() -> Directory {
        Directory { name: String::from(""), files: Vec::new(), subdirectories: Vec::new() }
    }
}

struct File {
    name: String,
    size: usize
}