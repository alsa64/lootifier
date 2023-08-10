use clap::Parser;
use serde_yaml::to_string as to_yaml_string;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::{Path, PathBuf};

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

fn read_file_to_vector(path: &PathBuf) -> io::Result<Vec<String>> {
    let lines = BufReader::new(File::open(path)?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    Ok(lines
        .iter()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty() && !line.starts_with('#') && !line.starts_with('/'))
        .collect())
}

fn escape_string(string: &str) -> String {
    string.replace('\'', "''")
}

#[derive(serde::Serialize)]
struct LootRule {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,
}

fn generate_rules(plugins: &[String]) -> io::Result<String> {
    let escaped_plugins: Vec<_> = plugins.iter().map(|plugin| escape_string(plugin)).collect();

    // Use zip and windows for more efficient iteration without index manipulation
    let loot_groups_rules: Vec<_> = escaped_plugins
        .windows(2)
        .map(|window| LootRule {
            name: window[1].clone(),
            after: Some(window[0].clone()),
            group: None,
        })
        .collect();

    let loot_plugin_rules: Vec<_> = escaped_plugins
        .iter()
        .map(|plugin| LootRule {
            name: plugin.clone(),
            after: None,
            group: Some(plugin.clone()),
        })
        .collect();

    let mut rules_map = HashMap::new();
    rules_map.insert("groups", &loot_groups_rules);
    rules_map.insert("plugins", &loot_plugin_rules);

    to_yaml_string(&rules_map).map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))
}

fn write_string_to_file(string: &str, path: &Path) -> io::Result<()> {
    File::create(path)?.write_all(string.as_bytes())
}

fn clear_file(masterlist_path: &Path) -> io::Result<()> {
    if masterlist_path.exists() {
        write_string_to_file("", masterlist_path)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let arguments = Opt::parse();

    let plugins = read_file_to_vector(&arguments.input)?;

    let output_string = generate_rules(&plugins)?;

    println!("{}", output_string);
    write_string_to_file(&output_string, &arguments.output)?;

    clear_file(&arguments.masterlist_path)?;

    Ok(())
}
