use colored::*;
use std::collections::HashMap;
use std::fs;
use std::num::Wrapping;

use crate::error::Error;
use crate::parser::{Parser, Program};
use crate::tape::Tape;
use crate::utils::{getchar, Output};

type ProgramCounter = usize;
type BracketsMap = HashMap<ProgramCounter, ProgramCounter>;
// type ErrorWithExtraInfo = (Error, ProgramCounter);

pub struct Modes {
    pub debug: bool,
    pub numerical: bool,
    pub big_int: bool,
}

pub struct Interpreter {
    modes: Modes,
    tape: Tape,
    stack: Vec<ProgramCounter>,
    brackets: BracketsMap,
    pub program_counter: ProgramCounter,
    pub output: Output,
}

impl Interpreter {
    pub fn new(modes: Modes) -> Self {
        Self {
            modes,
            tape: Tape::new(),
            program_counter: 0,
            brackets: HashMap::new(),
            stack: Vec::new(),
            output: Output::Stdout(std::io::stdout()),
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

    fn handle_comma(&mut self) -> Result<(), Error> {
        let input = getchar();
        match input {
            Ok(v) => Ok(self.tape.set_current_value(Wrapping(v.into()))),
            Err(e) => Err(Error::Runtime(e)),
        }
    }

    fn handle_dot(&mut self, numerical_mode: bool) {
        if numerical_mode {
            self.output.write(format!("{}\n", self.tape.current_value));
        } else {
            self.output.write(format!(
                "{}",
                char::from_u32(self.tape.current_value.0 as u32)
                    .expect("big-int mode was used without numerical mode!")
            ));
        }
    }

    fn enter_loop(&mut self) {
        if self.tape.current_value.0 == 0 {
            self.program_counter = self.brackets[&self.program_counter];
        } else {
            self.stack.push(self.program_counter);
        }
    }

    fn leave_loop(&mut self) {
        match self.stack.last() {
            Some(v) => {
                if self.tape.current_value.0 == 0 {
                    self.stack.pop();
                } else {
                    self.program_counter = *v;
                }
            }
            // it will fail earlier if there is an
            // ending bracket but not an opening one
            None => unreachable!(),
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
        // TODO: PROVIDE BETTER EXAMPLE

        while let Some(instruction) = program.next_instruction() {
            // match *instruction {};
            // if let Err(e) = res {
            //     let tb = Traceback::traceback(&program, self.program_counter, e);
            //     return Err(tb);
            // }
            // match instruction.get_op() {}

            // we want to print newline at the end but
            // not when in numerical mode because it already prints one
            // TODO: if we run in debug mode and there was no output
            // we will get an extra empty line, this isn't necesserially desired
            // if !self.modes.numerical {
            //     self.output.write(format!(""));
            // }
            // if self.modes.debug {
            //     self.output.write(format!(
            //         "----{}-----\n{}\n",
            //         "DEBUG INFO".yellow(),
            //         self.tape
            //     ));
            // }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_interpreter;
