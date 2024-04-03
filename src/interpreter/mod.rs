use colored::*;
use std::io::Write;
use std::num::Wrapping;
use std::{fs, io::Read};

use crate::{
    parser::{instruction::Operation, Parser, Program},
    tape::Tape,
};

pub struct Interpreter<'a, R: Read, W: Write> {
    tape: Tape,
    // for some reason it's neccessary to have this as mutable because .read() takes that?
    pub input: &'a mut R,
    pub output: &'a mut W,
}

impl<'a, R: Read, W: Write> Interpreter<'a, R, W> {
    pub fn new(input: &'a mut R, output: &'a mut W) -> Self {
        Self {
            tape: Tape::default(),
            input,
            output,
        }
    }

    fn handle_dot(&mut self, numerical_mode: bool) {
        // if this operation can't write to self.output then that's a critical error
        // hence the unwrapping
        if numerical_mode {
            self.output.write(&[self.tape.current_value.0]).unwrap();
        } else {
            self.output
                .write(self.tape.current_value.0.to_string().as_bytes())
                .unwrap();
        };
    }

    pub fn run<P: AsRef<std::path::Path>>(&mut self, file: P) -> Result<(), String> {
        let file_path = file.as_ref();

        let src = fs::read_to_string(file_path).map_err(|e| {
            format!(
                "Error occured while reading {}: {}",
                file_path.display().to_string().bold(),
                e
            )
        })?;

        let mut program = Parser::parse(&src).map_err(|e| {
            format!(
                "Error occured while parsing {}: {}",
                file_path.display().to_string().bold(),
                e
            )
        })?;
        self.execute(&mut program, false)
    }

    pub fn execute(&mut self, program: &mut Program, literal_output: bool) -> Result<(), String> {
        while let Some(instruction) = program.fetch_instruction() {
            match *instruction.get_op() {
                Operation::TapeLeft => self.tape.move_left(instruction.get_n())?,
                Operation::TapeRight => self.tape.move_right(instruction.get_n())?,
                Operation::TapePrint => {
                    // this is just debug information, so even if this fails it's not fatal
                    // and it's probably ok to just ignore it
                    let _ = self
                        .output
                        .write(format!("!TAPE: {}", self.tape).as_bytes());
                }
                Operation::CellInc => self.tape.inc(instruction.get_n()),
                Operation::CellDec => self.tape.dec(instruction.get_n()),
                Operation::CellRead => self.handle_dot(literal_output),
                Operation::CellWrite => {
                    let mut buf: [u8; 1] = [0];
                    self.input
                        .read_exact(&mut buf[..])
                        .map_err(|e| e.to_string())?;
                    self.tape.set_current_value(Wrapping(buf[0].into()));
                }
                Operation::BeginLoop(_) => program.begin_loop(self.tape.current_value.0.into()),
                Operation::EndLoop => program.end_loop(self.tape.current_value.0.into()),
            };
            program.inc_pc();
        }
        Ok(())
    }
}

#[cfg(test)]
mod interpreter_tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_basic_io() {
        let mut input = Cursor::new(vec![123]);
        let mut output: Vec<u8> = Vec::new();
        let mut interpreter = Interpreter::new(&mut input, &mut output);
        let mut program = Parser::parse(",+++.").unwrap();

        interpreter.execute(&mut program, true).unwrap();
        assert_eq!(output, vec![126]);
    }

    #[test]
    fn test_loops() {
        let mut input = Cursor::new(vec![10]);
        let mut output: Vec<u8> = Vec::new();
        let mut interpreter = Interpreter::new(&mut input, &mut output);
        let mut program = Parser::parse("[-].").unwrap();

        interpreter.execute(&mut program, true).unwrap();
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn test_tape_print() {
        let mut input = Cursor::new(vec![1, 2, 3]);
        let mut output: Vec<u8> = Vec::new();

        let mut interpreter = Interpreter::new(&mut input, &mut output);
        let mut program = Parser::parse(",>,>,!TAPE").unwrap();

        interpreter.execute(&mut program, false).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "!TAPE: 1 2 [3]");
    }
}
