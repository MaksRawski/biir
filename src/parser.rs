use std::fs;
use std::num::Wrapping;

use crate::tape::Tape;
use crate::utils::getchar;

pub struct Parser{
    pub tape: Tape,
    pub program_counter: usize,
    pub stack: Vec<usize>,
}

impl Parser{
    pub fn new() -> Self{
        Self{
            tape: Tape::new(),
            program_counter: 0,
            stack: Vec::new()
        }
    }

    // TODO: improve this
    pub fn find_closing_bracket(&self, program: &str) -> Result<usize, ()>{
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

    pub fn handle_comma(&mut self) -> Result<(), String>{
        let input = getchar();
        match input{
            Ok(v) => Ok( self.tape.set_current_value( Wrapping(v) )),
            Err(e) => Err( format!("Runtime error: {}", e) ),
        }
    }

    pub fn handle_dot(&mut self, numerical_mode: bool) -> Result<(), String>{
        if numerical_mode{
            println!("{}", self.tape.current_value);
        }
        else{
            print!("{}", self.tape.current_value.0 as char);
        }
        Ok( () )
    }


    pub fn enter_loop(&mut self, program: &str) -> Result<(), String>{
        if self.tape.current_value.0 == 0{
            match self.find_closing_bracket(program){
                Ok(v) => {
                    self.program_counter = v;
                    Ok( () )
                }
                Err(_) => Err( String::from("Syntax error: '[' doesn't have a matching closing bracket" ) )
            }
        }
        else{
            self.stack.push(self.program_counter);
            Ok( () )
        }
    }

    pub fn leave_loop(&mut self) -> Result<(), String>{
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
                Err( String::from("Syntax error: ']' doesn't have a matching opening bracket!") )
            }
        }
    }

    pub fn run(&mut self, file: &str, numerical_mode: bool, debug_mode: bool) -> Result<(), String>{
        let program: String;
        match fs::read_to_string(file){
            Ok(v) => program = v,
            Err(_) => return Err( format!("File {} could not be read", file) ),
        }
        self.execute(&program, numerical_mode, debug_mode)
    }

    pub fn execute(&mut self, program: &str, numerical_mode: bool, debug_mode: bool) -> Result<(), String>{
        while self.program_counter < program.len(){
            let error: Result<(), String> = match program.chars().nth(self.program_counter).unwrap_or(' '){
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
                        println!("!TAPE: {}", self.tape);
                    }
                    Ok( () )
                },
                _ => Ok( () ),
            };
            self.program_counter += 1;

            // we return only if there was an error
            if let Err(e) = error{
                if debug_mode{
                    // TODO: this could definietly be cleaned up, improved
                    // print current program line or 100 chars in both directions
                    let min = std::cmp::max(self.program_counter as i32 - 100, 1) as usize;
                    let max = std::cmp::min(self.program_counter as i32 + 100, program.len() as i32) as usize;

                    let start_of_line: usize = &program[min..self.program_counter]
                        .rfind('\n')
                        .unwrap_or(min) - 1;
                    let end_of_line: usize = &self.program_counter + &program[self.program_counter..max]
                        .find('\n')
                        .unwrap_or(max);

                    let current_line = &program[start_of_line..end_of_line];
                    let line_nr = &program[0..self.program_counter].lines().count();
                    let pointer = " ".repeat(self.program_counter - start_of_line - 1) + "^";
                    let err_msg = format!("{}\non line {}, char {}:\n{}\n{}\nTAPE: \n{}", e, line_nr, self.program_counter - start_of_line, current_line, pointer, self.tape);
                    return Err(err_msg);
                }
                else{
                    return Err(e);
                }
            }
        }
        // we want to print newline at the end but
        // not when in numerical mode because it already prints one
        // TODO: if we run in debug mode and there was no output
        // we will get an extra empty line, this isn't necesserially desired
        if !numerical_mode{
            println!("");
        }
        if debug_mode{
            println!("----DEBUG INFO-----\n{}", self.tape);

        }
        Ok( () )
    }
}
