use colored::*;
use std::collections::HashMap;
use std::fs;
use std::num::Wrapping;

use crate::error::Error;
use crate::tape::Tape;
use crate::traceback::Traceback;
use crate::unicodes::*;
use crate::utils::{getchar, Output};

type ProgramCounter = usize;
type BracketsMap = HashMap<ProgramCounter, ProgramCounter>;
type ErrorWithExtraInfo = (Error, ProgramCounter);

pub struct Modes {
    pub debug: bool,
    pub numerical: bool,
}

pub struct Parser {
    tape: Tape,
    stack: Vec<ProgramCounter>,
    brackets: BracketsMap,
    pub program_counter: ProgramCounter,
    pub output: Output,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tape: Tape::new(),
            program_counter: 0,
            brackets: HashMap::new(),
            stack: Vec::new(),
            output: Output::Stdout(std::io::stdout()),
        }
    }

    fn process_char(&mut self, chr: char, modes: &Modes) -> Result<Option<String>, Error> {
        match chr {
            '-' => self.tape.dec(),
            '+' => self.tape.inc(),
            '<' => return self.tape.move_left().map(|_| None),
            '>' => return self.tape.move_right().map(|_| None),
            ',' => return self.handle_comma().map(|_| None),
            '.' => self.handle_dot(modes.numerical),
            '[' => self.enter_loop(),
            ']' => self.leave_loop(),
            '!' => {
                if modes.debug {
                    return Ok(Some(format!("{}: {}\n", "!TAPE".yellow(), self.tape)));
                }
            }
            _ => (),
        }
        Ok(None)
    }

    fn find_brackets(program: &str) -> Result<BracketsMap, ErrorWithExtraInfo> {
        let mut brackets = BracketsMap::new();
        let mut stack = Vec::new();

        for (i, current_char) in program.chars().enumerate() {
            if current_char == '[' {
                stack.push(i);
            } else if current_char == ']' {
                let last_stack_value = stack.pop();
                if last_stack_value.is_none() {
                    return Err((
                        Error::Syntax(
                            "Closing bracket doesn't have a matching opening bracket!".to_string(),
                        ),
                        i,
                    ));
                } else {
                    brackets.insert(last_stack_value.unwrap(), i);
                }
            }
        }

        // if there still is an opening bracket
        // that doesn't have a matching ending bracket
        let last_stack_value = stack.pop();
        dbg!(last_stack_value);

        if last_stack_value.is_some() && !brackets.contains_key(&last_stack_value.unwrap()) {
            Err((
                Error::Syntax(
                    "Opening bracket doesn't have a matching closing bracket!".to_string(),
                ),
                last_stack_value.unwrap(),
            ))
        } else {
            Ok(brackets)
        }
    }

    fn handle_comma(&mut self) -> Result<(), Error> {
        let input = getchar();
        match input {
            Ok(v) => Ok(self.tape.set_current_value(Wrapping(v))),
            Err(e) => Err(Error::Runtime(e)),
        }
    }

    fn handle_dot(&mut self, numerical_mode: bool) {
        if numerical_mode {
            self.output.write(format!("{}\n", self.tape.current_value));
        } else {
            self.output
                .write(format!("{}", self.tape.current_value.0 as char));
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

    pub fn run<P: AsRef<std::path::Path>>(&mut self, file: P, modes: Modes) -> Result<(), String> {
        let program: String;
        let file_path = file.as_ref();

        match fs::read_to_string(file_path) {
            Ok(v) => program = v,
            Err(e) => {
                return Err(format!(
                    "Error occured while reading {}: {}",
                    file_path.display().to_string().bold(),
                    e
                ))
            }
        }
        self.execute(&program, modes)
    }

    pub fn execute(&mut self, program: &str, modes: Modes) -> Result<(), String> {
        self.program_counter = 0;

        let graphemes = program
            .graphemes(true)
            .filter(|c| !c.contains('\n'))
            .collect::<UnicodeString>();

        match Parser::find_brackets(program) {
            Ok(brackets) => self.brackets = brackets,
            Err((error, index)) => return Err(Traceback::traceback(&program, index, error)),
        }

        while self.program_counter < graphemes.len() {
            // iterate over graphemes but only process chars
            let current_char = graphemes
                .iter()
                .nth(self.program_counter)
                .unwrap()
                .chars()
                .next()
                .unwrap();

            let res: Result<Option<String>, Error> = self.process_char(current_char, &modes);

            // we return only if there was an error
            if let Err(e) = res {
                let tb = Traceback::traceback(&program, self.program_counter, e);
                return Err(tb);
            }

            self.program_counter += 1;
        }
        // we want to print newline at the end but
        // not when in numerical mode because it already prints one
        // TODO: if we run in debug mode and there was no output
        // we will get an extra empty line, this isn't necesserially desired
        if !modes.numerical {
            self.output.write(format!(""));
        }
        if modes.debug {
            self.output.write(format!(
                "----{}-----\n{}\n",
                "DEBUG INFO".yellow(),
                self.tape
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_parser;
