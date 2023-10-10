mod decompile;

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let ti_file = decompile::read_binary_file(&args[1]);

    println!("{}", decompile::decompile(ti_file.unwrap()).join("\n"));
}
