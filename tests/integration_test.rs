#[cfg(test)]
extern crate biir;

use colored::*;
use regex::Regex;
use std::fs;
use test_case::test_case;

use biir::parser::Parser;
use biir::utils::Output;

#[test_case( "!TAPE\n+!TAPE\n+!TAPE",
    r"(?m).*!TAPE.*: \[0\] \n.*!TAPE.*: \[1\] \n.*!TAPE.*: \[2\]" ; "!TAPE")]
fn test_output(program: &str, expected_output: &str) {
    let mut parser = Parser::new();
    parser.output = Output::Vector(Vec::new());

    let expected_output = Regex::new(expected_output).unwrap();

    assert_eq!(parser.execute(program, false, true), Ok(()));
    assert!(expected_output.is_match(&parser.output.read()));
}

#[test_case("[", r"Syntax error.*\n.*char 1:\n"; "Syntax error")]
#[test_case("<", r"Runtime error.*\n.*char 1:\n"; "Runtime error")]
fn test_error_messages(program: &str, expected_error_message: &str) {
    let mut parser = Parser::new();
    parser.output = Output::Vector(Vec::new());

    let expected_error_message = Regex::new(expected_error_message).unwrap();
    let error_msg = parser.execute(program, false, true).err().unwrap();

    assert!(expected_error_message.is_match(&error_msg));
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
