#[cfg(test)]
extern crate biir;

use std::fs;

use biir::parser::Parser;
use biir::utils::Output;

#[test]
fn test_example_programs() {
    for f in fs::read_dir("./tests/programs").unwrap() {
        let f = f.unwrap();
        let program = f.path();
        test_program(program);
    }
}

fn test_program(program: std::path::PathBuf) {
    dbg!(program.file_name().unwrap());
    let mut parser = Parser::new();
    parser.output = Output::Vector(Vec::new());

    // expected output is in the first line
    let expected_output = fs::read_to_string(&program)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .replace("\\n", "\n");

    assert_eq!(parser.run(&program, false, false), Ok(()));
    assert_eq!(format!("[{}]", parser.output.read()), expected_output);
}

// TODO: test program with newlines
// TODO: test program with windows style (\r\n) newlines
