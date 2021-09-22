use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "lootifier")]
struct Opt {
    /// Input ModOrganizer Loadorder
    #[structopt(name = "MO Loadorder Input Path", short = "i", long = "input", default_value = "loadorder.txt", parse(from_os_str))]
    input: PathBuf,

    /// Output file
    #[structopt(name = "Loot Userlist Output Path", short = "o", long = "output", default_value = "userlist.yaml", parse(from_os_str))]
    output: PathBuf,

    /// Masterlist path, if specified the file will be cleared
    #[structopt(name = "Loot Masterlist input Path", short = "m", long = "masterlist-input", default_value = "", parse(from_os_str))]
    masterlist_path: PathBuf,
}

fn main() {
    // collect CLI arguments
    let arguments = Opt::from_args();

    let plugins = load_lines_to_string_vector(&arguments.input);

    let output_string = generate_plugin_based_rules(plugins);

    // print userlist.yaml to stdout
    println!("{}", output_string);

    // write userlist.yaml to disk
    write_string_to_file(&arguments.output, output_string);

    // Check if the user specified a masterlist path, if so, write an empty file to that path
    if arguments.masterlist_path.to_str().expect("Could not convert masterlist_path to type str") != "" {
        write_string_to_file(&arguments.masterlist_path, String::new());
    }
}

fn generate_plugin_based_rules(plugins: Vec<String>) -> String {
    let plugins_len = plugins.len();

    // create userlist.yaml in memory as string
    let mut output_str = String::from("groups:\n    - name: \'default\'\nplugins:");
    for i in 1..plugins_len {
        let i = plugins_len - i;
        output_str.push_str("\n");
        output_str.push_str("  - name: \'");
        output_str.push_str(plugins[i].as_str());
        output_str.push_str("\'\n    after:\n      - \'");
        output_str.push_str(plugins[i - 1].as_str());
        output_str.push_str("\'");
    }
    output_str
}

/// Given an output path and a String, it will write it to that path as a file.
fn write_string_to_file(output_path: &PathBuf, output_str: String) {
    let output_display = output_path.display();
    let mut file = match File::create(&output_path) {
        Err(why) => panic!("couldn\'t create {}: {}", output_display, why),
        Ok(file) => file,
    };

    match file.write_all(output_str.as_bytes()) {
        Err(why) => panic!("couldn\'t write to {}: {}", output_display, why),
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
        let line = line.replace("\'", "\'\'");
        if line.starts_with('#') {
            continue;
        }
        lines.push(line);
    }
    lines
}