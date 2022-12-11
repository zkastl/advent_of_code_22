use std::{fs, ptr::null};

pub fn puzzle(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("\n### ADVENT OF CODE 2022: DAY 7 SOLUTIONS ###");

    let data = fs::read_to_string(filepath)?;
    let split: Vec::<&str> = data.split("\n").collect();

    let file_system = map_filetree(String::from("/"), split, Directory::default());

    Ok(data)
}

fn map_filetree(name: String, history: Vec::<&str>, mut directory: Directory) -> Directory {

    // Create the local map
    let mut local_map: Directory = Directory { 
        name: name,
        files: Vec::new(),
        subdirectories: Vec::new()
    };
    
    // Reach each line
    for line in 0..history.len() {
        let tokens: Vec::<&str> = history[line].split(" ").collect();
        match tokens[0] {
            "$" => {
                println!("Processing command: {} {}", tokens[0], tokens[1]);
                match tokens[1] {
                    "cd" => {
                        match tokens[2] {
                            ".." => {
                                return local_map;
                            }

                            "/" => {
                                continue;
                            }

                            &_ => {                               
                                local_map.subdirectories.push(
                                    map_filetree(tokens[2].to_string(),
                                    history[line..history.len()].to_vec(),
                                    Directory::default()));

                            }
                        }
                    }
                    "ls" => {
                        continue;
                    }
                    _ => println!("UNSUPPORTED COMMAND")
                }
            }

            "dir" => {
                println!("Skipping directory line...");
                continue;
            }

            _ => {
                println!("Found local file: {}", tokens[1]);
                local_map.files.push(
                    File {name: tokens[1].to_string(), size: tokens[0].parse::<usize>().unwrap() });
                    continue;

            }
        }
    }

    return local_map;
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