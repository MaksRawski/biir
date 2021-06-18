use std::io;
use std::io::{Read, Write};
use termion::raw::IntoRawMode;

pub fn getchar() -> Result<u8, String>{
    let mut buffer = [0];
    let stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = io::stdin();

    stdout.lock().flush().unwrap();

    // TODO: this looks like it could be done more concisely
    return if let Ok(_) = stdin.read_exact(&mut buffer){
        if buffer[0] == 3{
            Err( String::from("got Ctrl-C, Exiting!") )
        }
        else{
            Ok( buffer[0] )
        }
    }
    else{
        Err( String::from("not enough input given") )
    }
}

/// common interface for writing to stdout
/// as well as Vector, to allow integration tests
pub enum Output{
    Stdout(std::io::Stdout),
    Vector(Vec<String>),
}

impl Output{
    pub fn write(&mut self, msg: String){
        match self{
            Output::Stdout(out) => { out.write(msg.as_bytes()).unwrap(); },
            Output::Vector(out) => { out.push(msg) },
        };
    }
}


pub struct Traceback;

impl Traceback{
    /// returns info about current line as a tuple:
    /// (line_nr, char_nr, current_line)
    /// ```
    /// let (line_nr, char_nr, current_line) = Traceback::current_line("TEST 1\nTEST 2", 9);
    ///
    /// assert_eq!(line_nr, 2);
    /// assert_eq!(char_nr, 1);
    /// assert_eq!(current_line, "TEST 2");
    /// ```
    fn current_line<'a>(program: &'a str, program_counter: usize) -> (usize, usize, &'a str){
        let line_nr = &program[0..program_counter].lines().count();
        let chars_before: &usize = &program.lines()
            .take(*line_nr-1)
            .fold(0, |sum, l| sum + l.chars().count());

        let char_nr = program_counter - chars_before;
        let current_line = &program.lines().nth(line_nr - 1).unwrap();

        (*line_nr, char_nr, current_line)
    }

    /// returns entire line but with the current char red
    /// ```
    /// use colored::*;
    /// assert_eq!(
    ///     Traceback::highlight_current_char_in_line("Test 123", 4),
    ///     format!("Tes{} 123", "t".red())
    /// );
    ///
    fn highlight_current_char_in_line(current_line: &str, char_nr: usize) -> String{
        format!("{}{}{}",
            current_line.chars().take(char_nr).collect::<String>(),
            current_line.chars().nth(char_nr).unwrap().to_string().red(),
            current_line.chars().skip(char_nr).take(current_line.chars().count() - char_nr).collect::<String>(),
        )
    }

    pub fn traceback(program: &str, program_counter: usize, error_msg: &str) -> String{
        let (line_nr, char_nr, current_line) = Traceback::current_line(program, program_counter);
        let highlighted_current_line = Traceback::highlight_current_char_in_line(current_line, char_nr);

        format!(
            "{}\non line {}, char {}:\n{}\n",
            error_msg, line_nr, char_nr, highlighted_current_line
        )
    }
}
