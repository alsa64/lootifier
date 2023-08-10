use serde_yaml::to_string as to_yaml_string;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::{Path, PathBuf};
pub struct Lootifier {
    plugins: Vec<String>,
}

impl Lootifier {
    pub fn from_file(path: &PathBuf) -> io::Result<Self> {
        let plugins = Lootifier::read_file_to_vector(path)?;
        Ok(Lootifier { plugins })
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

    pub fn write_string_to_file(string: &str, path: &Path) -> io::Result<()> {
        File::create(path)?.write_all(string.as_bytes())
    }

    pub fn clear_file(masterlist_path: &Path) -> io::Result<()> {
        if masterlist_path.exists() {
            Self::write_string_to_file("", masterlist_path)?;
        }
        Ok(())
    }
}

#[derive(serde::Serialize)]
struct LootRule {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,
}
