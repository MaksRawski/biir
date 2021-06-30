use colored::*;

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

    /// indexes from 1
    fn char_number(program: &str, line_nr: usize, program_counter: usize) -> usize {
        // this should fail if there is mismatch between line_nr and program_counter
        // however we trust that it's fine and don't check that explicitly
        let chars_before_current_line: &usize = &program
            .lines()
            .take(line_nr)
            .fold(0, |sum, l| sum + l.chars().count());

        program_counter - chars_before_current_line
    }

    /// returns entire line but with the current char red
    /// will return an Error on empty string
    fn highlight_current_char_in_line(current_line: &str, char_nr: usize) -> Result<String, ()> {
        Ok(format!(
            "{}{}{}",
            current_line.chars().take(char_nr).collect::<String>(),
            current_line
                .chars()
                .nth(char_nr)
                .ok_or(())?
                .to_string()
                .red(),
            current_line
                .chars()
                .skip(char_nr + 1)
                .take(current_line.chars().count() - char_nr)
                .collect::<String>(),
        ))
    }

    pub fn traceback(program: &str, program_counter: usize, error_msg: &str) -> Result<String, ()> {
        let line_nr = Traceback::line_number(program, program_counter)?;
        let current_line = program.lines().nth(line_nr).ok_or(())?;
        let char_nr = Traceback::char_number(program, line_nr, program_counter);

        let highlighted_current_line =
            Traceback::highlight_current_char_in_line(current_line, char_nr)?;

        Ok(format!(
            "{}\non line {}, char {}:\n{}\n",
            error_msg, line_nr + 1, char_nr, highlighted_current_line
        ))
    }
}

#[cfg(test)]
mod test_traceback;
