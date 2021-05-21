use std::io;
use std::io::{Read, Write};

use termion::raw::IntoRawMode;

pub fn getchar() -> u8{
    let mut buffer = [0];
    let stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = io::stdin();

    stdout.lock().flush().unwrap();
    stdin.read_exact(&mut buffer).unwrap();

    buffer[0]
}

// TODO: improve this
pub fn find_closing_bracket(program_counter: usize, code: &str) -> Result<usize, ()>{
    let mut depth = 0;
    for i in program_counter..code.len(){
        let current_char = code.chars().nth(i).unwrap();
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
