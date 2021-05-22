use std::fs;
use std::num::Wrapping;

use crate::tape::Tape;
use crate::utils::getchar;

pub struct Parser{
    tape: Tape,
    program_counter: usize,
    stack: Vec<usize>,
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

    fn handle_comma(&mut self) -> Result<(), String>{
        Ok( self.tape.set_current_value( Wrapping(getchar()) ) )
    }

    fn handle_dot(&mut self, numerical_mode: bool) -> Result<(), String>{
        if numerical_mode{
            print!("{}", self.tape.current_value);
        }
        else{
            print!("{}", self.tape.current_value.0 as char);
        }
        Ok( () )
    }


    fn enter_loop(&mut self, program: &str) -> Result<(), String>{
        if self.tape.current_value.0 == 0{
            // find a matching closing bracket and move program counter +1 from there
            match self.find_closing_bracket(program){
                Ok(v) => {
                    self.program_counter = v;
                    Ok( () )
                }
                Err(_) => Err( format!("[char: {}] Syntax error: '[' doesn't have matching closing bracket", self.program_counter) )
            }
        }
        else{
            self.stack.push(self.program_counter);
            Ok( () )
        }
    }

    fn leave_loop(&mut self) -> Result<(), String>{
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
                Err( format!("[char: {}] Syntax error: ']' doesn't have matching opening bracket!", self.program_counter) )
            }
        }
    }

    pub fn run(&mut self, file: &str) -> Result<(), String>{
        let program: String;
        match fs::read_to_string(file){
            Ok(v) => program = v,
            Err(_) => return Err( format!("File {} could not be read", file) ),
        }
        self.execute(&program, false)
    }

    pub fn execute(&mut self, program: &str, numerical_mode: bool) -> Result<(), String>{
        while self.program_counter < program.len(){
            let error: Result<(), String> = match program.chars().nth(self.program_counter).unwrap(){
                '-' => Ok( self.tape.dec() ),
                '+' => Ok( self.tape.inc() ),
                '<' => self.tape.move_left(),
                '>' => self.tape.move_right(),
                ',' => self.handle_comma(),
                '.' => self.handle_dot(numerical_mode),
                '[' => self.enter_loop(&program),
                ']' => self.leave_loop(),
                _ => Ok( () ),
            };
            self.program_counter += 1;

            // we return only if there was an error
            if let Err(e) = error{
                return Err(e);
            }
        }
        Ok( () )
    }

    pub fn debug(&mut self){
        print!("{}", self.tape);
    }
}
