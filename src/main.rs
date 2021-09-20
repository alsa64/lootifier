use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::path::{Display, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "lootifier")]
struct Opt {
    /// Input Modorganizer Loadorder
    #[structopt(name = "MO Loadorder Input Path", short = "i", long = "input", default_value = "loadorder.txt", parse(from_os_str))]
    input: PathBuf,

    /// Output file
    #[structopt(name = "Loot Userlist Output Path", short = "o", long = "output", default_value = "userlist.yaml", parse(from_os_str))]
    output: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    // input and output paths
    let loadorder_path = opt.input;
    let output_path = opt.output;

    let plugins = load_lines_to_string_vector(&loadorder_path);
    let output_display = output_path.display();

    let plugins_len = plugins.len();

    // create userlist.yaml in memory as string
    let mut output_str = String::from("plugins:\n");
    for i in 1..plugins_len {
        output_str.push_str("  - name '");
        output_str.push_str(plugins[i].as_str());
        output_str.push_str("'\n    after: ['");
        output_str.push_str(plugins[i - 1].as_str());
        output_str.push_str("']\n\n");
    }

    // print userlist.yaml to stdout
    println!("{}", output_str);

    // write userlist.yaml to disk
    write_string_to_file(&output_path, output_display, output_str)
}

/// Given an output path and a String, it will write it to that path as a file.
fn write_string_to_file(output_path: &PathBuf, output_display: Display, output_str: String) {
    let mut file = match File::create(&output_path) {
        Err(why) => panic!("couldn't create {}: {}", output_display, why),
        Ok(file) => file,
    };

    match file.write_all(output_str.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", output_display, why),
        Ok(_) => println!("successfully wrote to {}", output_display),
    }
}

/// Loads input_file_path and returns it as a String vector, seperated by new lines.
/// It ignores all lines that start with #
fn load_lines_to_string_vector(input_file_path: &PathBuf) -> Vec<String> {
    let input_file = File::open(input_file_path).expect("Unable to open file");
    let input_file_buf = BufReader::new(input_file);
    let mut lines: Vec<String> = Vec::new();
    for line in input_file_buf.lines() {
        let line = line.expect("Unable to read line");
        if line.starts_with("#") {
            continue;
        }
        lines.push(line);
    }
    lines
}