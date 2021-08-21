use clap::{App, Arg};
use std::process;

use biir::parser::Parser;
use biir::parser::Modes;

fn main() {
    let args = App::new("BIIR")
        .version("1.2.2")
        .author("Maks Rawski <maksymilian.rawski@tutanota.com>")
        .about("Brainfuck Interpreter In Rust")
        .arg(Arg::with_name("file")
            .required(true)
        )
        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug-mode")
            .help("Allows the usage of `!TAPE` to print 10 nearby tape values,\nwill also print some debug info at the end")
        )
        .arg(Arg::with_name("numerical")
            .short("n")
            .long("numerical-mode")
            .help("Prints byte values instead of their ascii representations")
        )
        .get_matches();

    let mut parser = Parser::new();
    let file = args.value_of("file").unwrap();

    let modes = Modes{
        debug: args.is_present("debug"),
        numerical: args.is_present("numerical"),
    };

    if let Err(e) = parser.run(file, modes) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
