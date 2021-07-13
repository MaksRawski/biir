use std::fs;
use std::num::Wrapping;
use colored::*;

use crate::tape::Tape;
use crate::utils::{getchar, Output};
use crate::traceback::Traceback;
use crate::error::Error;
use crate::unicodes::*;

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

    /// returns offset from current program_counter
    /// to the matching closing bracket
    fn find_closing_bracket(&self, program: &UnicodeString) -> Result<usize, ()>{
        let mut depth = 0;
        let program = program.iter().skip(self.program_counter);

        for (i, current_char) in program.enumerate(){
            if *current_char == "["{
                depth += 1;
            }
            else if *current_char == "]"{
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


    fn enter_loop(&mut self, program: &UnicodeString) -> Result<(), Error>{
        if self.tape.current_value.0 == 0{
            match self.find_closing_bracket(program){
                Ok(v) => {
                    self.program_counter = self.program_counter + v;
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

    pub fn run<P: AsRef<std::path::Path>>(&mut self, file: P, numerical_mode: bool, debug_mode: bool) -> Result<(), String>{
        let program: String;
        let file_path = file.as_ref();

        match fs::read_to_string(file_path){
            Ok(v) => program = v,
            Err(e) => return Err(
                format!("Error occured while reading {}: {}",
                    file_path.display().to_string().bold(),
                e)
            ),
        }
        self.execute(&program, numerical_mode, debug_mode)
    }

    pub fn execute(&mut self, program: &str, numerical_mode: bool, debug_mode: bool) -> Result<(), String>{
        self.program_counter = 0;

        let graphemes = program
            .graphemes(true)
            .filter(|c| !c.contains('\n'))
            .collect::<UnicodeString>();

        while self.program_counter < graphemes.len(){
            let current_char = graphemes.iter().nth(self.program_counter);

            let error: Result<(), Error> = match *current_char.unwrap_or(&&" "){
                "-" => Ok( self.tape.dec() ),
                "+" => Ok( self.tape.inc() ),
                "<" => self.tape.move_left(),
                ">" => self.tape.move_right(),
                "," => self.handle_comma(),
                "." => self.handle_dot(numerical_mode),
                "[" => self.enter_loop(&graphemes),
                "]" => self.leave_loop(),
                "!" => {
                    let next_5_chars = &graphemes[self.program_counter..self.program_counter+5].concat();

                    if debug_mode && *next_5_chars == "!TAPE"{
                        self.output.write( format!("{}: {}\n", "!TAPE".yellow(), self.tape) );
                    }
                    Ok( () )
                },
                _ => Ok( () ),
            };

            // we return only if there was an error
            if let Err(e) = error{
                let error_msg = match e{
                    Error::Syntax(msg) => format!("{} {}", "Syntax error:".red(), msg.normal()),
                    Error::Runtime(msg) => format!("{} {}", "Runtime error:".red(), msg.normal()),
                };
                let tb = Traceback::traceback(&program, self.program_counter, &error_msg);
                return Err( tb.unwrap_or(format!("Error occured while trying to print traceback")) );
            }

            self.program_counter += 1;
        }
        // we want to print newline at the end but
        // not when in numerical mode because it already prints one
        // TODO: if we run in debug mode and there was no output
        // we will get an extra empty line, this isn't necesserially desired
        if !numerical_mode{
            self.output.write( format!("") );
        }
        if debug_mode{
            self.output.write( format!("----{}-----\n{}\n", "DEBUG INFO".yellow(), self.tape) );
        }
        Ok( () )
    }
}

#[cfg(test)]
mod test_parser;
