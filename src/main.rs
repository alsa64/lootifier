use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "lootifier")]
struct Opt {
    /// Input ModOrganizer Loadorder
    #[structopt(name = "input path", short = "i", long = "input", default_value = "loadorder.txt", parse(from_os_str))]
    input: PathBuf,

    /// Output file
    #[structopt(name = "output path", short = "o", long = "output", default_value = "", parse(from_os_str))]
    output: PathBuf,

    /// Masterlist path, if specified the file will be cleared
    #[structopt(name = "clear path", short = "m", long = "masterlist-input", default_value = "", parse(from_os_str))]
    masterlist_path: PathBuf,

    /// Use Plugin based Sorting (LOOT)
    #[structopt(name = "bool", short = "p", long = "plugin-sort")]
    use_plugin_sort: bool,


    /// Use Group based Sorting (LOOT)
    #[structopt(name = "bool", short = "g", long = "group-sort")]
    use_group_sort: bool,
}

fn main() {
    // collect CLI opts
    let opt = Opt::from_args();

    let plugins = load_lines_to_string_vector(&opt.input);

    let mut output_path_string = output;

    let output_string = 
    if opt.use_plugin_sort == true {
        if output_path_string == "" {
            output_path_string = "userlist.yaml";
        }
        generate_plugin_based_rules(plugins)
    } else if opt.use_group_sort == true {
        if output_path_string == "" {
            output_path_string = "userlist.yaml";
        }
        generate_group_based_rules(plugins)
    } else {
        output_path_string = "Skyrim.ccc";
        generate_skyrim_ccc(plugins)
    };

    // print userlist.yaml to stdout
    println!("{}", output_string);

    // write userlist.yaml to disk
    write_string_to_file(&opt.output, output_string);

    // Check if the user specified a masterlist path, if so, write an empty file to that path
    if opt.masterlist_path.to_str().expect("Could not convert masterlist_path to type str") != "" {
        write_string_to_file(&opt.masterlist_path, String::new());
    }
}

fn generate_plugin_based_rules(plugins: Vec<String>) -> String {
    let plugins_len = plugins.len();

    // create userlist.yaml in memory as string
    let mut output_str = String::from("groups:\n    - name: \'default\'\nplugins:");
    for i in 1..plugins_len {
        let i = plugins_len - i;
        let current_plugin = plugins[i].as_str();
        let load_after_plugin = plugins[i - 1].as_str();
        output_str.push_str(format!("\n").as_str());
        output_str.push_str(format!("  - name: \'{}\'\n", current_plugin).as_str());
        output_str.push_str(format!("    after:\n").as_str());
        output_str.push_str(format!("      - \'{}\'", load_after_plugin).as_str());
    }
    output_str
}

fn generate_group_based_rules(plugins: Vec<String>) -> String {
    let plugins_len = plugins.len();

    let mut groups_str = String::from("groups:\n");
    let mut plugins_str = String::from("plugins:");
    let mut output = String::from("");
    for i in 1..plugins_len {
        let i = plugins_len - i;
        let current_plugin = plugins[i].as_str();
        let load_after_plugin = plugins[i - 1].as_str();

        groups_str.push_str(format!("  - name: \'{}\'\n", current_plugin).as_str());
        if load_after_plugin != "Skyrim.esm" {
            groups_str.push_str(format!("    after:\n").as_str());
            groups_str.push_str(format!("      - \'{}\'\n", load_after_plugin).as_str());
        }

        plugins_str.push_str(format!("\n").as_str());
        plugins_str.push_str(format!("  - name: \'{}\'\n", current_plugin).as_str());
        plugins_str.push_str(format!("    group: \'{}\'", current_plugin).as_str());
    }
    output.push_str(groups_str.as_str());
    output.push_str(plugins_str.as_str());
    output
}

fn generate_skyrim_ccc(plugins: Vec<String>) -> String {
    let plugins_len = plugins.len();

    let mut output_str = String::from("");
    for i in 1..plugins_len {
        output_str.push_str(format!("{}\n", plugins[i].as_str()).as_str());
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

