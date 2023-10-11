use std::fs;

use clap::arg;
use log::error;
use tio2::{utils, decompile, parse::lex};

fn main() {
    env_logger::init();

    // define the CLI interface
    let matches = clap::command!()
        .args(&[
            arg!(<INFILE> "The input file. Can be a .8Xp file or decompiled TI-BASIC text."),
            arg!(-d --decompile [OUTFILE] "Only decompile and write to an output file. Defaults to stdout.")
        ])
        .get_matches();

    // Attempt to read the content of the specified input file
    let file_data = match utils::read_file_bytes(matches.get_one::<String>("INFILE").unwrap()) {
        Ok(v) => v, // Success, store the file data
        Err(e) => {
            // Error, log the message and exit the program with an 1
            error!("Could not read file: {}", e);
            std::process::exit(1);
        }
    };

    // Check if the file data is valid UTF-8 or not
    let ti_file_string = if utils::is_utf8(file_data.clone()) {
        match String::from_utf8(file_data) {
            Ok(v) => v, // valid UTF-8, store the data as a UTF-8 encoded string
            Err(e) => {
                // Error, log the message and exit the program with an 1
                error!("Could not convert string to UTF-8: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // If the file data is not valid UTF-8, attempt to decompile it
        match decompile::decompile(file_data) {
            Ok(v) => v.join("\n"), // Success, join the result into a string
            Err(e) => {
                // Error, log the message and exit the program with an 1
                error!("Could not decompile 8Xp file: {}", e);
                std::process::exit(1);
            }
        }
    };

    // Check if the `decompile` flag is specified
    if matches.contains_id("decompile") {
        let outfile = match matches.get_one::<String>("decompile") {
            Some(v) => v,
            None => {
                // If no output file is specified, print to stdout and exit
                println!("{}", ti_file_string);
                std::process::exit(0);
            },
        };

        // Write the decompiled content to the specified output file
        match fs::write(outfile, ti_file_string) {
            Ok(_) => (),
            Err(e) => {
                error!("Unable to write file: {}", e);
                std::process::exit(1);
            },
        }
    } else {
        println!("{:?}", lex("If X=45"));
    }
}
