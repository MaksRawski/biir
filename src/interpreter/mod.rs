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

    fn handle_dot(&mut self) {
        let numerical_mode = false;
        if numerical_mode {
            let _ = self
                .output
                .write(format!("{}\n", self.tape.current_value).as_bytes());
        } else {
            let _ = self.output.write(
                format!(
                    "{}",
                    char::from_u32(self.tape.current_value.0 as u32)
                        .expect("big-int mode was used without numerical mode!")
                )
                .as_bytes(),
            );
        }
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
        self.execute(&mut program)
    }

    pub fn execute(&mut self, program: &mut Program) -> Result<(), String> {
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
                Operation::CellRead => self.handle_dot(),
                Operation::CellWrite => {
                    let mut buf: [u8; 1] = [0];
                    self.input
                        .read_exact(&mut buf[..])
                        .map_err(|e| e.to_string())?;
                    self.tape.set_current_value(Wrapping(buf[0].into()));
                }
                Operation::BeginLoop(_) => program.begin_loop(self.tape.current_value.0),
                Operation::EndLoop => program.end_loop(self.tape.current_value.0),
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
        let mut out: Vec<u8> = Vec::new();
        let mut interpreter = Interpreter::new(&mut input, &mut out);
        let mut program = Parser::parse(",+++.").unwrap();

        interpreter.execute(&mut program).unwrap();
        assert_eq!(out, vec![126]);
    }

    #[test]
    fn test_loops() {
        let mut out: Vec<u8> = Vec::new();
        let mut input = Cursor::new(vec![10]);
        let mut interpreter = Interpreter::new(&mut input, &mut out);
        let mut program = Parser::parse("[-].").unwrap();

        interpreter.execute(&mut program).unwrap();
        assert_eq!(out, vec![0]);
    }
}
