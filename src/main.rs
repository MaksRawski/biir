use clap::{App, Arg};
use std::process;

use biir::interpreter::Interpreter;

fn main() {
    let args = App::new("BIIR")
        .version("1.3.0")
        .author("Maks Rawski <maksymilian.rawski@tutanota.com>")
        .about("Brainfuck Interpreter In Rust")
        .arg(Arg::with_name("file")
            .required(true)
        )
        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug-mode")
            .help("Allows the usage of `!TAPE` to print 10 nearby tape values,\nwill also print some debug info at the end.")
        )
        .arg(Arg::with_name("numerical")
            .short("n")
            .long("numerical-mode")
            .help("Prints byte values instead of their ascii representations.")
        )
        .arg(Arg::with_name("big int")
            .short("b")
            .long("big-int-mode")
            .help("Uses raw usize for storing cell value instead of u8 with wrapping.\nWorks only when numerical mode is used!")
        )
        .get_matches();

    let file = args.value_of("file").unwrap();
    if args.is_present("big int") {
        eprintln!("Big int mode is only available when using --numerical-mode");
        process::exit(1);
    }

    let i = &mut std::io::stdin();
    let o = &mut std::io::stdout();
    let mut interpreter = Interpreter::new(i, o);

    if let Err(e) = interpreter.run(file) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
