use std::fs;

use biir::{interpreter::Interpreter, parser::Parser};

#[cfg(test)]
extern crate biir;

#[test]
fn test_hello_world() {
    let mut input = std::io::empty();
    let mut output = Vec::<u8>::new();
    let mut interpreter = Interpreter::new(&mut input, &mut output);
    let src = fs::read_to_string("tests/programs/hello_world.bf").unwrap();
    let mut program = Parser::parse(&src).unwrap();
    interpreter.execute(&mut program, true).unwrap();
    let expected_output = src
        .lines()
        .next()
        .unwrap()
        .trim_matches(|c| c == '[' || c == ']')
        .replace("\\n", "\n");

    assert_eq!(output, expected_output.as_bytes())
}
