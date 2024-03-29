use colored::*;
use getchar::getchar;
use std::fs;
use std::io::Write;
use std::num::Wrapping;

use crate::{
    parser::{Operation, Parser, Program},
    tape::Tape,
};

// type ErrorWithExtraInfo = (Error, ProgramCounter);

pub struct Config {
    pub debug: bool,
    pub numerical: bool,
    pub big_int: bool,
}

pub struct Interpreter<'a, W: Write> {
    config: Config,
    tape: Tape,
    program: Program,
    pub output: &'a mut W,
}

impl<'a, W: Write> Interpreter<'a, W> {
    pub fn new(modes: Config, out: &'a mut W) -> Self {
        Self {
            config: modes,
            tape: Tape::new(),
            program: Program::new(),
            output: out,
        }
    }

    // fn process_char(&mut self, chr: char) -> Result<Option<String>, Error> {
    //     match chr {
    //         '-' => self.tape.dec(),
    //         '+' => self.tape.inc(),
    //         '<' => return self.tape.move_left().map(|_| None),
    //         '>' => return self.tape.move_right().map(|_| None),
    //         ',' => return self.handle_comma().map(|_| None),
    //         '.' => self.handle_dot(self.modes.numerical),
    //         '[' => self.enter_loop(),
    //         ']' => self.leave_loop(),
    //         '!' => {
    //             if self.modes.debug {
    //                 return Ok(Some(format!("{}: {}\n", "!TAPE".yellow(), self.tape)));
    //             }
    //         }
    //         _ => (),
    //     }
    //     Ok(None)
    // }

    fn handle_dot(&mut self, numerical_mode: bool) {
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
        let src: String;
        let file_path = file.as_ref();

        match fs::read_to_string(file_path) {
            Ok(v) => src = v,
            Err(e) => {
                return Err(format!(
                    "Error occured while reading {}: {}",
                    file_path.display().to_string().bold(),
                    e
                ))
            }
        }
        let mut program = Parser::parse(&src)?;
        self.execute(&mut program)
    }

    pub fn execute(&mut self, program: &mut Program) -> Result<(), String> {
        // let mut opening_brackets_stack: Vec<Position> = Vec::new();
        // TODO: let's keep brackets map at runtime only
        // let's have a vector that stores a beginnging bracket of a given level
        // at a given index e.g. vec![1, 3, 7] would be created for:
        // +[.[<+>[->]]] this makes no sense btw.
        // NOPE!!! We need to (ideally) store a map of opening to closing brackets, so that
        // when we don't enter the loop we go to the matching closing bracket

        while let Some(instruction) = program.next_instruction() {
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
                Operation::CellRead => match getchar() {
                    Some(c) => self.tape.set_current_value(Wrapping((c as u8).into())),
                    None => return Err("Failed to getchar!".to_string()),
                },

                Operation::CellWrite => self.handle_dot(self.config.numerical),
                Operation::BeginLoop(_) => self.program.begin_loop(self.tape.current_value.0),
                Operation::EndLoop => self.program.end_loop(self.tape.current_value.0),
            };
        }
        return Ok(());
    }
}
