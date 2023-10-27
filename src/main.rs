use std::{fs, process};

use clap::{arg, ArgGroup};
use log::error;
use tio2::{
    interpreter,
    translation::{common::TIFile, compile, decompile},
    utils,
};

fn main() {
    env_logger::init();

    // define the CLI interface
    let matches = clap::command!()
        .args(&[
            arg!(-r --run <INFILE> "Interpret an input file. Can be a .8XP file or decompiled TI-BASIC text."),
            arg!(-d --decompile <INFILE> "Decompile an input file and write to an output file. Defaults to stdout."),
            arg!(-c --compile <INFILE> "Compile a TI-BASIC text file into an 8XP file.").requires("name"),
            arg!(-o --out <OUTFILE> "Specify a file to output to, if applicable (decompilation)."),
            arg!(-n --name <NAME> "Specify the program name to use when compiling."),
        ])
        .group(
            ArgGroup::new("action")
            .args(["run", "decompile", "compile"])
            .required(true),
        )
        .get_matches();

    // Attempt to read the content of the specified input file
    let filename = if let Some(filename) = matches.get_one::<String>("decompile") {
        filename
    } else if let Some(filename) = matches.get_one::<String>("compile") {
        filename
    } else if let Some(filename) = matches.get_one::<String>("run") {
        filename
    } else {
        error!("Something has gone terribly wrong and the infile name couldn't be read");
        process::exit(1);
    };

    // TODO: extract these into functions
    if matches.contains_id("decompile") {
        let file_data = match utils::read_file_bytes(filename) {
            Ok(v) => v, // Success, store the file data
            Err(e) => {
                // Error, log the message and exit the program with an 1
                error!("Could not read file: {}", e);
                process::exit(1);
            }
        };

        let ti_file_string = match decompile::decompile(file_data) {
            Ok(v) => v.join("\n"), // Success, join the result into a string
            Err(e) => {
                // Error, log the message and exit the program with an 1
                error!("Could not decompile 8Xp file: {}", e);
                process::exit(1);
            }
        };

        // We're decompiling a given input file
        let outfile = match matches.get_one::<String>("out") {
            Some(v) => v,
            None => {
                // If no output file is specified, print to stdout and exit
                println!("{}", ti_file_string);
                process::exit(0);
            }
        };

        // Write the decompiled content to the specified output file
        match fs::write(outfile, ti_file_string) {
            Ok(_) => (),
            Err(e) => {
                error!("Unable to write file: {}", e);
                process::exit(1);
            }
        }
    } else if matches.contains_id("compile") {
        let file_data = match utils::read_file_lines(filename) {
            Ok(v) => v,
            Err(e) => {
                error!("Could not read file: {}", e);
                process::exit(1);
            }
        };

        let program_name = match matches.get_one::<String>("name") {
            Some(v) => {
                if !v.chars().all(|c| c.is_ascii_alphabetic()) {
                    error!("Name argument is not ASCII Alphabetic.");
                    process::exit(1);
                }
                v
            }
            None => {
                error!("Name argument was not provided.");
                process::exit(1);
            }
        };

        let res = match compile::compile_to_bytecode(file_data.iter().map(|s| s.as_str()).collect())
        {
            Ok(v) => v,
            Err(e) => {
                error!("Error when compiling: {}", e);
                process::exit(1);
            }
        };

        let (header, footer) = match compile::create_metadata(&res, program_name) {
            Ok((h, f)) => (h, f),
            Err(e) => {
                error!("Error when compiling: {}", e);
                process::exit(1);
            }
        };

        let ti_file = TIFile {
            header,
            data: res,
            footer: footer.to_vec(),
        };

        match ti_file.write_to_file() {
            Ok(_) => (),
            Err(e) => {
                error!("Error when writing to file: {}", e);
                process::exit(1);
            }
        };
    } else if matches.contains_id("run") {
        let file_data = match utils::read_file_bytes(filename) {
            Ok(v) => v, // Success, store the file data
            Err(e) => {
                // Error, log the message and exit the program with an 1
                error!("Could not read file: {}", e);
                process::exit(1);
            }
        };

        let ti_program = match decompile::read_binary_data(file_data) {
            Ok(v) => v,
            Err(e) => {
                error!("Could not parse binary data: {}", e);
                process::exit(1);
            }
        };

        let mut bytecode = interpreter::Interpreter::new(&ti_program).unwrap();
        bytecode.parse_bytes();
        println!("{:?}", bytecode);
    }

    // // Check if the file data is valid UTF-8 or not
    // let ti_file_string = if utils::is_utf8(file_data.clone()) {
    //     match String::from_utf8(file_data) {
    //         Ok(v) => v, // valid UTF-8, store the data as a UTF-8 encoded string
    //         Err(e) => {
    //             // Error, log the message and exit the program with an 1
    //             error!("Could not convert string to UTF-8: {}", e);
    //             process::exit(1);
    //         }
    //     }
    // } else {
    //     // If the file data is not valid UTF-8, attempt to decompile it
    // match decompile::decompile(file_data) {
    //     Ok(v) => v.join("\n"), // Success, join the result into a string
    //     Err(e) => {
    //         // Error, log the message and exit the program with an 1
    //         error!("Could not decompile 8Xp file: {}", e);
    //         process::exit(1);
    //     }
    // }
    // };

    // // Check if the `decompile` flag is specified
    // if matches.contains_id("decompile") {
    // } else {
    //     let res = match compile::compile_to_bytecode(vec![
    //         "ClrHome\n",
    //         "Input \"WEIGHT \",W\n",
    //         "Input \"HEIGHT \",H\n",
    //         "W*H*9.8→X\n",
    //         "ClrHome\n",
    //         "Disp X",
    //     ]) {
    //         Ok(v) => v,
    //         Err(e) => {
    //             error!("Error when compiling: {}", e);
    //             process::exit(1);
    //         }
    //     };

    //     let (header, footer) = match compile::create_metadata(&res, "gpe") {
    //         Ok((h, f)) => (h, f),
    //         Err(e) => {
    //             error!("Error when compiling: {}", e);
    //             process::exit(1);
    //         }
    //     };

    //     let ti_file = TIFile {
    //         header,
    //         data: res,
    //         footer: footer.to_vec(),
    //     };

    //     println!("{:?}", ti_file.write_to_file());
    // }
}
