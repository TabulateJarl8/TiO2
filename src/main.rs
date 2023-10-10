use clap::arg;
use log::error;

mod decompile;
pub mod utils;

fn main() {
    env_logger::init();

    let matches = clap::command!()
        .args(&[
            arg!(<INFILE> "The input file. Can be a .8Xp file or decompiled TI-BASIC text."),
            arg!(-d --decompile "Decompile the 8Xp file only, don't interpret it."),
        ])
        .get_matches();

    let file_data = match utils::read_file_bytes(matches.get_one::<String>("INFILE").unwrap()) {
        Ok(v) => v,
        Err(e) => {
            error!("Could not read file: {}", e);
            std::process::exit(1);
        }
    };
    let ti_file_string = if utils::is_utf8(file_data.clone()) {
        match String::from_utf8(file_data) {
            Ok(v) => v,
            Err(e) => {
                error!("Could not convert string to UTF-8: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        match decompile::decompile(file_data) {
            Ok(v) => v.join("\n"),
            Err(e) => {
                error!("Could not decompile 8Xp file: {}", e);
                std::process::exit(1);
            }
        }
    };

    println!("{}", ti_file_string);
}
