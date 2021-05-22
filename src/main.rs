use biir::parser::Parser;
use std::process;
// use clap;

fn main() {
    let mut parser = Parser::new();

    match parser.run("test.bf"){
        Ok( () ) => {},
        Err( e ) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
