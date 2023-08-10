use clap::Parser;
use lootifier::Lootifier;
use std::{env, io, path::PathBuf}; // Change this import

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

fn run_cli() -> io::Result<()> {
    let arguments = Opt::parse();

    let lootifier = Lootifier::from_file(&arguments.input)?;

    let output_string = lootifier.generate_rules()?;

    println!("{}", output_string);
    Lootifier::write_string_to_file(&output_string, &arguments.output)?;

    Lootifier::clear_file(&arguments.masterlist_path)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        run_cli()?;
    } else {
        run_cli()?; //TODO: Replace this with function that starts a GUI for lootifier
    }
    Ok(())
}
