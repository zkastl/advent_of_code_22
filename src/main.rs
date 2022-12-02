use std::fs;

fn main() {
    println!("Hello, world!");

    read_file_string("C:\\Users\\zkast\\source\\repos\\advent_of_code_22\\in1a.txt").expect("help!");
    
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("entered function");
    let data = fs::read_to_string(filepath)?;
    //println!("{}",data);

    let split = data.split("\n");
    let mut elves = Vec::new();
    elves.push(0);
    for s in split {
        let pos = elves.len() - 1;
        if s != "\n" {
            elves[pos] += 1;
            //println!("{}",s);
        }
        else {
            println!("{}\n", elves[pos].to_string());
            elves.push(0)
        }
    };
    Ok(data)
}