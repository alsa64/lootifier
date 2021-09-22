# lootifier

Converts a Mod Organizer loadorders to loot rules for manual loadorders

## Long Description

This tool is meant for people who want to share modpacks on platforms like Wabbajack that have a) a custom loadorder and b) optional mods.

While you can provide instructions for users where to put the optional plugins or you could create your own custom loot rules manually,
this process is quite tedious. With this tool, you can automatically convert your loadorder into a set of custom loot rules and optionally
clear the loot masterlist to prevent it from conflicting with your own custom loadorder.

## Usage

To use this tool, either put it in the current profile folder that has all plugins activated or
use the arguments outlined below to automatically read and write from the correct paths. 

```
lootifier 1.1.0

USAGE:
    lootifier.exe [FLAGS] [OPTIONS]

FLAGS:
    -p, --plugin-sort    Use Plugin based Sorting instead of Group based sorting
    -h, --help           Prints help information
    -V, --version        Prints version information

OPTIONS:
    -m, --masterlist-input <clear path>    Masterlist path, if specified the file will be cleared [default: ]
    -i, --input <input path>               Input ModOrganizer Loadorder [default: loadorder.txt]
    -o, --output <output path>             Output file [default: userlist.yaml]
    ```

## Build instructions

1. [Install Rust](https://www.rust-lang.org/learn/get-started)
2. Open a Terminal in the repository folder.
3. Run:
```bash
cargo build --release
```
