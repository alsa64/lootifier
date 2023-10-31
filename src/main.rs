use std::path::PathBuf;

use clap::Parser;

// use lootifier::file_util;
use lootifier::Lootifier;

mod file_util;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Parse command line arguments using the `Opt` structure.
    let arguments = Opt::parse();

    // 2. Create a new `Lootifier` object using the input file specified in the arguments.
    let lootifier = Lootifier::from_file(&arguments.input)?;

    // 3. Generate loot rules using the `Lootifier` object.
    let output_string = lootifier.generate_rules()?;

    // 4. Print the generated rules to the console.
    println!("{}", output_string);

    // 5. Save the generated rules to the specified output file.
    file_util::write_string_to_file(&output_string, &arguments.output)?;

    // 6. Clear the contents of the masterlist file (if it exists) as specified in the arguments.
    file_util::clear_file(&arguments.masterlist_path)?;

    // 7. Return success (Ok).
    Ok(())
}
