use std::{env, path::Path};

use files::convert_file;

use crate::files::open_file;

mod files;

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

    for arg in &cmd_args {
        let arg = arg.as_str();

        if (arg == "-convert" || arg == "-c") && cmd_args.len() >= 3 {
            convert_file(Path::new(&cmd_args[1]), Path::new(&cmd_args[2])).unwrap();
            return;
        }

        if cmd_args.len() == 1 {
            open_file(Path::new(&cmd_args[0])).unwrap();
            return;
        }

        println!("{}", HELP_MSG);

    }

}