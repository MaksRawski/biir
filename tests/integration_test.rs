#[cfg(test)]
extern crate biir;

use colored::*;
use regex::Regex;
use std::fs;

use biir::parser::Parser;
use biir::utils::Output;

#[test]
fn test_tape() {
    let mut parser = Parser::new();
    parser.output = Output::Vector(Vec::new());
    let program = "!TAPE\n+!TAPE\n+!TAPE";
    // using .* to make up for color
    let expected_output =
        Regex::new(r"(?m).*!TAPE.*: \[0\] \n.*!TAPE.*: \[1\] \n.*!TAPE.*: \[2\]").unwrap();

    assert_eq!(parser.execute(program, false, true), Ok(()));
    assert!(expected_output.is_match(&parser.output.read()));
}

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
