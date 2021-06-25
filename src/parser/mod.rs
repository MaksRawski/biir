use std::fs;
use std::num::Wrapping;
use colored::*;

use crate::tape::Tape;
use crate::utils::{getchar, Output};
use crate::traceback::Traceback;
use crate::error::Error;

pub struct Parser{
    tape: Tape,
    stack: Vec<usize>,
    pub program_counter: usize,
    pub output: Output,
}

impl Parser{
    pub fn new() -> Self{
        Self{
            tape: Tape::new(),
            program_counter: 0,
            stack: Vec::new(),
            output: Output::Stdout(std::io::stdout()),
        }
    }

    // TODO: improve this
    fn find_closing_bracket(&self, program: &str) -> Result<usize, ()>{
        let mut depth = 0;
        for i in self.program_counter..program.len(){
            let current_char = program.chars().nth(i).unwrap();
            if current_char == '['{
                depth += 1;
            }
            else if current_char == ']'{
                depth -= 1;
                if depth == 0{
                    return Ok(i);
                }
            }
        }
        Err( () )
    }

    fn handle_comma(&mut self) -> Result<(), Error>{
        let input = getchar();
        match input{
            Ok(v) => Ok( self.tape.set_current_value( Wrapping(v) )),
            Err(e) => Err( Error::Runtime(e) ),
        }
    }

    fn handle_dot(&mut self, numerical_mode: bool) -> Result<(), Error>{
        if numerical_mode{
            self.output.write( format!("{}\n", self.tape.current_value) );
        }
        else{
            self.output.write( format!("{}", self.tape.current_value.0 as char) );
        }
        Ok( () )
    }


    fn enter_loop(&mut self, program: &str) -> Result<(), Error>{
        if self.tape.current_value.0 == 0{
            match self.find_closing_bracket(program){
                Ok(v) => {
                    self.program_counter = v;
                    Ok( () )
                }
                Err(_) => Err( Error::Syntax("'[' doesn't have a matching closing bracket!".to_string()) )
            }
        }
        else{
            self.stack.push(self.program_counter);
            Ok( () )
        }
    }

    fn leave_loop(&mut self) -> Result<(), Error>{
        match self.stack.last(){
            Some(v) => {
                if self.tape.current_value.0 == 0{
                    self.stack.pop();
                }
                else{
                    self.program_counter = *v;
                }
                Ok( () )
            },
            None => {
                Err( Error::Syntax("']' doesn't have a matching opening bracket!".to_string()) )
            }
        }
    }

    pub fn run(&mut self, file: &str, numerical_mode: bool, debug_mode: bool) -> Result<(), String>{
        let program: String;
        match fs::read_to_string(file){
            Ok(v) => program = v,
            Err(_) => return Err( format!("File {} could not be read!", file) ),
        }
        self.execute(&program, numerical_mode, debug_mode)
    }

    pub fn execute(&mut self, program: &str, numerical_mode: bool, debug_mode: bool) -> Result<(), String>{
        // TODO: iterate over graphemes instead
        while self.program_counter < program.len(){
            let error: Result<(), Error> = match program.chars().nth(self.program_counter).unwrap_or(' '){
                '-' => Ok( self.tape.dec() ),
                '+' => Ok( self.tape.inc() ),
                '<' => self.tape.move_left(),
                '>' => self.tape.move_right(),
                ',' => self.handle_comma(),
                '.' => self.handle_dot(numerical_mode),
                '[' => self.enter_loop(&program),
                ']' => self.leave_loop(),
                '!' => {
                    if debug_mode && program[self.program_counter..self.program_counter+5] == *"!TAPE"{
                        self.output.write( format!("!TAPE: {}", self.tape) );
                    }
                    Ok( () )
                },
                _ => Ok( () ),
            };
            self.program_counter += 1;

            // we return only if there was an error
            if let Err(e) = error{
                let error_msg = match e{
                    Error::Syntax(msg) => format!("{} {}", "Syntax error".red(), msg.normal()),
                    Error::Runtime(msg) => format!("{} {}", "Runtime error".red(), msg.normal()),
                };
                let tb = Traceback::traceback(program, self.program_counter, &error_msg);
            }
        }
        // we want to print newline at the end but
        // not when in numerical mode because it already prints one
        // TODO: if we run in debug mode and there was no output
        // we will get an extra empty line, this isn't necesserially desired
        if !numerical_mode{
            self.output.write( format!("") );
        }
        if debug_mode{
            self.output.write( format!("----DEBUG INFO-----\n{}", self.tape) );
        }
        Ok( () )
    }
}

#[cfg(test)]
mod test_parser;
