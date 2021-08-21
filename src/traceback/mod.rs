use crate::unicodes::*;
use colored::*;

use crate::error::Error;

pub struct Traceback;

impl Traceback {
    /// indexes from 0
    fn line_number(program: &str, program_counter: usize) -> Result<usize, ()> {
        let mut acc = program_counter as isize;
        let mut current_line: isize = -1;

        // keep subtracting the number of chars in line
        // until we've subtracted more chars
        // than there are in program_counter
        while acc >= 0 {
            acc -= program
                .lines()
                .nth((current_line + 1) as usize)
                .ok_or(())?
                .chars()
                .count() as isize;

            current_line += 1;
        }

        Ok(current_line as usize)
    }

    /// indexes from 0
    fn char_number(program: &str, line_nr: usize, program_counter: usize) -> usize {
        // this will fail if there is mismatch between line_nr and program_counter
        // however we trust that it's fine and don't check that explicitly
        let chars_before_current_line: &usize = &program
            .lines()
            .take(line_nr)
            .fold(0, |sum, l| sum + l.chars().count());

        program_counter - chars_before_current_line
    }

    /// returns entire line but with the current char red
    /// will return an Error on empty string
    fn highlight_current_char_in_line<'a>(
        current_line: &'a UnicodeString,
        char_nr: usize,
    ) -> Result<String, ()> {
        // it may seem as fold is a very costy way of collecting
        // but it is acutally pretty quick
        // https://play.rust-lang.org/?version=nightly&mode=release&edition=2018&gist=77ccd7e84e8c4c9f827d7b04711c94fb
        let before_current_char = current_line
            .iter()
            .take(char_nr)
            .fold(String::new(), |acc, x| acc + x);

        let current_char = current_line.iter().nth(char_nr).ok_or(())?.red();

        let after_current_char = current_line
            .iter()
            .skip(char_nr + 1)
            .take(current_line.len() - char_nr)
            .fold(String::new(), |acc, x| acc + x);

        Ok(format!(
            "{}{}{}",
            before_current_char, current_char, after_current_char
        ))
    }

    pub fn traceback(program: &str, program_counter: usize, error: Error) -> String {
        let line_nr = Traceback::line_number(program, program_counter).unwrap();
        let current_line = program.lines().nth(line_nr).ok_or(()).unwrap();
        let current_line = string_to_unicode_string(current_line);
        let char_nr = Traceback::char_number(program, line_nr, program_counter);

        let highlighted_current_line =
            Traceback::highlight_current_char_in_line(&current_line, char_nr).unwrap();

        format!(
            "{}\non line {}, char {}:\n{}\n",
            error.msg(),
            line_nr + 1,
            char_nr + 1,
            highlighted_current_line
        )
    }
}

#[cfg(test)]
mod test_traceback;
