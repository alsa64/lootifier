use serde_yaml::to_string as to_yaml_string;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

/// This module contains utility functions related to file operations.
pub mod file_util {
    use super::*;

    const COMMENT_PREFIXES: [&str; 2] = ["#", "/"];

    /// Reads lines from a file, filters comments and returns a Vec<String>.
    pub fn read_lines_from_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
        let lines = BufReader::new(File::open(path)?)
            .lines()
            .collect::<Result<Vec<_>, _>>()?;
        Ok(lines
            .iter()
            .map(|line| line.trim().to_string())
            .filter(|line| {
                !line.is_empty()
                    && !COMMENT_PREFIXES
                        .iter()
                        .any(|&prefix| line.starts_with(prefix))
            })
            .collect())
    }

    /// Writes a string to a specified file.
    pub fn write_string_to_file<P: AsRef<Path>>(string: &str, path: P) -> io::Result<()> {
        File::create(path)?.write_all(string.as_bytes())
    }

    /// Clears the content of a specified file.
    pub fn clear_file<P: AsRef<Path>>(masterlist_path: P) -> io::Result<()> {
        if masterlist_path.as_ref().exists() {
            write_string_to_file("", &masterlist_path)?;
        }
        Ok(())
    }
}

/// Main struct for generating Loot rules.
pub struct Lootifier {
    plugins: Vec<String>,
}

impl Lootifier {
    /// Creates a new `Lootifier` instance from a file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let plugins = file_util::read_lines_from_file(&path)?;
        Ok(Lootifier { plugins })
    }

    /// Escapes characters within a string.
    fn escape_string(string: &str) -> String {
        string.replace('\'', "''")
    }

    /// Generates group rules based on the plugins.
    fn generate_group_rules(escaped_plugins: &[String]) -> Vec<LootRule> {
        escaped_plugins
            .windows(2)
            .map(|window| LootRule {
                name: window[1].clone(),
                after: Some(window[0].clone()),
                group: None,
            })
            .collect()
    }

    /// Generates plugin rules based on the plugins.
    fn generate_plugin_rules(escaped_plugins: &[String]) -> Vec<LootRule> {
        escaped_plugins
            .iter()
            .map(|plugin| LootRule {
                name: plugin.clone(),
                after: None,
                group: Some(plugin.clone()),
            })
            .collect()
    }

    /// Generates Loot rules in YAML format.
    pub fn generate_rules(&self) -> io::Result<String> {
        let escaped_plugins: Vec<_> = self
            .plugins
            .iter()
            .map(|plugin| Self::escape_string(plugin))
            .collect();

        let loot_groups_rules = Self::generate_group_rules(&escaped_plugins);
        let loot_plugin_rules = Self::generate_plugin_rules(&escaped_plugins);

        let mut rules_map = HashMap::new();
        rules_map.insert("groups", &loot_groups_rules);
        rules_map.insert("plugins", &loot_plugin_rules);

        to_yaml_string(&rules_map)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))
    }
}

/// Struct for holding Loot rules.
#[derive(serde::Serialize)]
struct LootRule {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,
}
