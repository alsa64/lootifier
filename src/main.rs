use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(name = "lootifier")]
struct Opt {
    /// Input ModOrganizer Loadorder
    #[clap(
        name = "input path",
        short = 'i',
        long = "input",
        default_value = "loadorder.txt"
    )]
    input: PathBuf,

    /// Output file
    #[clap(
        name = "output path",
        short = 'o',
        long = "output",
        default_value = "userlist.yaml"
    )]
    output: PathBuf,

    /// Masterlist path, if specified the file will be cleared
    #[clap(
        name = "clear path",
        short = 'm',
        long = "masterlist-input",
        default_value = "masterlist.yaml"
    )]
    masterlist_path: PathBuf,
}

/// function that reads a file and returns a vector of strings
/// each string is a line in the file
fn read_file(path: &PathBuf) -> Vec<String> {
    let file = File::open(path).expect("File not found");
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    lines
}

/// function that reads a vector of strings and trimms whitespaces from each string at the beginning and the end or each line
fn trim_whitespaces(lines: Vec<String>) -> Vec<String> {
    let mut trimmed_lines = Vec::new();
    for line in lines {
        trimmed_lines.push(line.trim().to_string());
    }
    trimmed_lines
}

/// function that reads a vector of strings and removes all empty strings from the vector as well as strings starting with # or //
fn remove_empty_and_comments(lines: Vec<String>) -> Vec<String> {
    let mut trimmed_lines = Vec::new();
    for line in lines {
        if !line.is_empty() && !line.starts_with('#') && !line.starts_with('/') {
            trimmed_lines.push(line);
        }
    }
    trimmed_lines
}

/// function that reads a text file to a vector of strings, removing empty strings and comments and trimming whitespaces from each string
fn read_file_to_vector(path: &PathBuf) -> Vec<String> {
    let lines = read_file(path);
    let trimmed_lines = trim_whitespaces(lines);
    remove_empty_and_comments(trimmed_lines)
}

/// function that given a string escapes it for yaml
/// for example ' -> ''
fn escape_string(string: &str) -> String {
    let mut escaped_string = String::new();
    for c in string.chars() {
        if c == '\'' {
            escaped_string.push_str("''");
        } else {
            escaped_string.push(c);
        }
    }
    escaped_string
}

/// function that writes a string to a file
fn write_string_to_file(string: String, path: &PathBuf) {
    let mut file = File::create(path).expect("File not found");
    file.write_all(string.as_bytes())
        .expect("Could not write to file");
}

/// function that given an array of strings combines them into a single string
fn combine_strings(lines: Vec<String>) -> String {
    let mut combined_string = String::new();
    for line in lines {
        combined_string.push_str(&line);
    }
    combined_string
}

/// function that generates plugin sorting rules for LOOT in yaml syntax
fn generate_rules(plugins: &Vec<String>) -> String {
    // escape all plugins
    let mut escaped_plugins = Vec::new();
    for plugin in plugins {
        escaped_plugins.push(escape_string(plugin));
    }

    let mut loot_groups_rules: Vec<String> = Vec::new();
    let mut loot_plugin_rules: Vec<String> = Vec::new();

    loot_groups_rules.push("groups:\n".to_string());
    loot_plugin_rules.push("plugins:\n".to_string());

    // interate though all plugins and create loot groups and loot plugin rules
    // current plugin is the plugin that is currently being processed
    // load after plugin is the plugin that is loaded before the current plugin
    // in the very first run, load after plugin doesn't exist, so no rule get's created for it
    // examples:
    // plugins:
    // ```
    // Skyrim.esm
    // Update.esm
    // Dawnguard.esm
    // ```
    // group rules:
    // ```
    //   - name: 'Skyrim.esm'
    //   - name: 'Update.esm'
    //     after: 'Skyrim.esm'
    //   - name: 'Dawnguard.esm'
    //     after: 'Update.esm'
    // ```
    // plugin rules:
    // ```
    //   - name: 'Skyrim.esm'
    //     group: 'Skyrim.esm'
    //   - name: 'Update.esm'
    //     group: 'Update.esm'
    //   - name: 'Dawnguard.esm'
    //     group: 'Dawnguard.esm'
    //```
    for (current_plugin_index, current_plugin) in escaped_plugins.iter().enumerate() {
        let load_after_plugin = if current_plugin_index > 0 {
            escaped_plugins.get(current_plugin_index - 1).unwrap()
        } else {
            ""
        };
        loot_groups_rules.push(format!("  - name: '{}'\n", current_plugin));
        if !load_after_plugin.is_empty() {
            loot_groups_rules.push(format!("    after: '{}'\n", load_after_plugin));
        }
        loot_plugin_rules.push(format!("  - name: '{}'\n", current_plugin));
        loot_plugin_rules.push(format!("    group: '{}'\n", current_plugin));
    }

    // combine the rules into a single String
    let loot_groups_rules_string = combine_strings(loot_groups_rules);
    let loot_plugin_rules_string = combine_strings(loot_plugin_rules);

    // combine the rules into a single String
    format!("{}\n{}", loot_groups_rules_string, loot_plugin_rules_string)
}

fn main() {
    // collect CLI arguments
    let arguments = Opt::parse();

    let plugins = read_file_to_vector(&arguments.input);

    let output_string = generate_rules(&plugins);

    // print userlist.yaml to stdout
    println!("{}", output_string);

    // write userlist.yaml to file
    write_string_to_file(output_string, &arguments.output);

    // Check if the user specified a masterlist path, if so, write an empty file to that path
    if arguments
        .masterlist_path
        .to_str()
        .expect("Could not convert masterlist_path to type str")
        != ""
    {
        write_string_to_file("".to_string(), &arguments.masterlist_path);
    }
}
