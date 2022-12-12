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
    
    let mut line: usize = 0;
    // Reach each line
    while line < history.len() {
        println!("{}", history[line]);
        let tokens: Vec::<&str> = history[line].split(" ").collect();
        match tokens[0] {
            "$" => {
                match tokens[1] {
                    "cd" => {
                        match tokens[2] {
                            ".." => {
                                return local_map;
                            }

                            "/" => {
                            }

                            &_ => {   
                                let new_history = history[line+1..history.len()].to_vec();                            
                                local_map.subdirectories.push(
                                    map_filetree(tokens[2].to_string(),
                                    new_history,
                                    Directory::default()));

                            }
                        }
                    }
                    "ls" => {
                        let mut j: usize = line+1;
                        while j < history.len() {
                            let tokens: Vec::<&str> = history[line].split(" ").collect();
                            match tokens[0] {
                                "$" => {
                                    line += j;
                                    break;
                                }

                                "dir" => {
                                    local_map.subdirectories.push(
                                        Directory {
                                            name: tokens[1].to_string(),
                                            files: Vec::new(),
                                            subdirectories: Vec::new()
                                        });
                                }

                                &_ => local_map.files.push(File {
                                    name: tokens[1].to_string(),
                                    size: tokens[0].parse::<usize>().unwrap(),
                                }),

                            }

                            j += 1;
                        }
                    }
                    _ => println!("UNSUPPORTED COMMAND")
                }
            }

            _ => {

            }
        }

        line += 1;
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