use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use std::path::Path;

#[allow(dead_code)]
pub(crate) const COMMENT_PREFIXES: [&str; 2] = ["#", "/"];

/// This module contains utility functions related to file operations.
/// Reads lines from a file, filters comments and returns a Vec<String>.
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn write_string_to_file<P: AsRef<Path>>(string: &str, path: P) -> io::Result<()> {
    File::create(path)?.write_all(string.as_bytes())
}

/// Clears the content of a specified file.
#[allow(dead_code)]
pub fn clear_file<P: AsRef<Path>>(masterlist_path: P) -> io::Result<()> {
    if masterlist_path.as_ref().exists() {
        write_string_to_file("", &masterlist_path)?;
    }
    Ok(())
}

/// Filters out comment lines and empty lines from an iterator and returns a Vec<String>.
#[allow(dead_code)]
pub fn filter_lines<'a, I>(lines: I) -> Vec<String>
    where
        I: Iterator<Item=&'a str>,
{
    lines
        .map(|line| line.trim().to_string())
        .filter(|line| {
            !line.is_empty()
                && !COMMENT_PREFIXES
                .iter()
                .any(|&prefix| line.starts_with(prefix))
        })
        .collect()
}
