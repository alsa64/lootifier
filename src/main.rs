use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let loadorder = File::open("loadorder.txt").expect("Unable to open file");
    let loadorder = BufReader::new(loadorder);
    let mut plugins : Vec<String> = Vec::new();
    let output_path = Path::new("userlist.yaml");
    let output_display = output_path.display();
    for line in loadorder.lines() {
        let line = line.expect("Unable to read line");
        if line.starts_with("# This file") {
            continue;
        }
        plugins.push(line);
    }
    let len = plugins.len();
    let mut output_str = String::from("plugins:\n");
    for i in 1..len {
        output_str.push_str("  - name '");
        output_str.push_str(plugins[i].as_str());
        output_str.push_str("'\n    after: ['");
        output_str.push_str(plugins[i-1].as_str());
        output_str.push_str("']\n\n");
    }

    println!("{}", output_str);

    let mut file = match File::create(&output_path) {
        Err(why) => panic!("couldn't create {}: {}", output_display, why),
        Ok(file) => file,
    };
    
    match file.write_all(output_str.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", output_display, why),
        Ok(_) => println!("successfully wrote to {}", output_display),
    }
}