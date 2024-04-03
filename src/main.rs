use biir::parser::Parser;
use clap::{App, Arg};
use rustyline::{error::ReadlineError, DefaultEditor};
use std::process;

use biir::interpreter::Interpreter;

fn main() {
    let args = App::new("BIIR")
        .version("2.0.0")
        .author("Maks Rawski <maksymilian.rawski@tutanota.com>")
        .about("Brainfuck Interpreter In Rust")
        .arg(Arg::with_name("file"))
        .get_matches();

    if args.is_present("file") {
        let file = args.value_of("file").unwrap();

        let i = &mut std::io::stdin();
        let o = &mut std::io::stdout();
        let mut interpreter = Interpreter::new(i, o);

        if let Err(e) = interpreter.run(file) {
            eprintln!("{}", e);
            process::exit(1);
        }
    } else {
        let mut i = &mut std::io::stdin();
        let mut o = &mut std::io::stdout();
        let mut rl = DefaultEditor::new().unwrap();
        let mut interpreter = Interpreter::new(&mut i, &mut o);

        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str()).unwrap();
                    match Parser::parse(&line) {
                        Ok(mut prog) => {
                            if let Err(e) = interpreter.execute(&mut prog, false) {
                                eprintln!("{}", e);
                            } else {
                                // force flush the output from the interpreter
                                println!();
                            }
                        }
                        Err(e) => eprintln!("{}", e),
                    };
                    continue;
                }
                Err(ReadlineError::Interrupted) => {
                    break;
                }
                Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
    }
}
