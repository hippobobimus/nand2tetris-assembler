use nand2tetris_assembler::Config;
use env_logger;
use std::env;
use std::process;

fn main() {
    env_logger::init();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    log::debug!("Config generated from arguments\n{:#?}", config);

    if let Err(e) = nand2tetris_assembler::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
