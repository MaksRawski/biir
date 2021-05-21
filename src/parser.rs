use std::fs;
use std::num::Wrapping;

use crate::tape::Tape;
use crate::utils::{getchar, find_closing_bracket};

pub struct Parser{
    tape: Tape,
}

impl Parser{
    pub fn new() -> Self{
        Self{
            tape: Tape::new(),
        }
    }

//     /// cleans up the file, removes comments
//     pub fn parse(&self, file: &str) -> Result<ParsedProgram, io::Error >{
//         let program = fs::read_to_string(file)?;
//         let pattern = Regex::new(r"[^,.+-<>\[\]]*").unwrap();
//         let parsed = ParsedProgram{
//             program:  pattern.replace_all(&program, "").to_owned().to_string(),
//         };

//         Ok( parsed )
//     }

    // TODO: move logic out of this so that we can just pass a string and execute it
    pub fn run(&mut self, file: &str, numerical_mode: bool) -> Result<(), String>{
        let mut code: String;
        let mut program_counter = 0;
        let mut stack = Vec::new();

        match fs::read_to_string(file){
            Ok(v) => code = v,
            Err(_) => return Err( format!("File {} could not be read", file) ),
        }

        while program_counter < code.len(){
            let error: Result<(), String> = match code.chars().nth(program_counter).unwrap(){
                '-' => Ok( self.tape.dec() ),
                '+' => Ok( self.tape.inc() ),
                '<' => self.tape.move_left(),
                '>' => self.tape.move_right(),
                ',' => Ok( self.tape.set_current_value( Wrapping(getchar()) ) ),
                '.' => {
                    if numerical_mode{
                        print!("{}", self.tape.current_value);
                    }
                    else{
                        print!("{}", self.tape.current_value.0 as char);
                    }
                    Ok( () )
                }

                '[' => {
                    if self.tape.current_value.0 == 0{
                        // find a matching closing bracket and move program counter +1 from there
                        match find_closing_bracket(program_counter, &code){
                            Ok(v) => {
                                program_counter = v;
                                Ok( () )
                            }
                            Err(_) => Err( format!("[char: {}] Syntax error: '[' doesn't have matching closing bracket", program_counter) )
                        }
                    }
                    else{
                        stack.push(program_counter);
                        Ok( () )
                    }
                }

                ']' => {
                    match stack.last(){
                        Some(v) => {
                            if self.tape.current_value.0 == 0{
                                stack.pop();
                            }
                            else{
                                program_counter = *v;
                            }
                            Ok( () )
                        },
                        None => {
                            Err( format!("[char: {}] Syntax error: ']' doesn't have matching opening bracket!", program_counter) )
                        }
                    }
                }

                _ => Ok( () ),
            };
            program_counter += 1;

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
