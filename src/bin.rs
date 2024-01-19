use std::{env, path::Path};

use crate::files::{convert_file, open_file};

mod files;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_MSG: &str = "
USAGE: uwu-codec [options] {target_file}

Options:
    -c or -convert: Takes a target and output file argument and encodes the target file to the output file. E.g. uwu-codec -c apple.png apple.uwu

Flags:
    --help: Shows this message.
    --version: Shows current version.
";

fn main() {
    let cmd_args: Vec<String> = env::args().skip(1).collect();

    let binding = "--help".to_owned();
    let option = cmd_args.get(0).unwrap_or(&binding);

    if (option == "-convert" || option == "-c") && cmd_args.len() >= 3 {
        let target_file = Path::new(&cmd_args[1]);
        let output_file = Path::new(&cmd_args[2]);

        println!("Converting {:?} to {:?}...", target_file, output_file);

        match convert_file(target_file, output_file) {
            Ok(_) => println!("Converted successfully!"),
            Err(e) => println!("An error occurred while converting! Error: {}", e),
        }

        return;
    }

    if option == "--help" {
        println!("{}", HELP_MSG);
        return;
    }

    if option == "--version" {
        println!("Version: v{}", VERSION);
        return;
    }

    if cmd_args.len() == 1 {
        let file = Path::new(&cmd_args[0]);

        if file.exists() {
            println!("Opening {:?}...", file);

            match open_file(Path::new(&cmd_args[0])) {
                Ok(_) => println!("Opened successfully!"),
                Err(e) => println!("An error occurred while opening that. Error: {}", e),
            }

            return;
        }

        println!("The file {:?} doesn't exist!", file);
    }

    println!("{}", HELP_MSG);

}