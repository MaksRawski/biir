use colored::*;

pub struct Traceback;

impl Traceback{
    /// returns info about current line as a tuple:
    /// (line_nr, char_nr, current_line)
    /// indexes from 0
    fn current_line<'a>(program: &'a str, program_counter: usize) -> (usize, usize, &'a str){
        let line_nr = &program[0..program_counter].lines().count() - 1;
        let current_line = &program.lines().nth(line_nr).unwrap();

        let chars_before_current_line: &usize = &program.lines()
            .take(line_nr)
            .fold(0, |sum, l| sum + l.chars().count());

        let index_of_char_in_current_line = program_counter - chars_before_current_line;

        (line_nr, index_of_char_in_current_line, current_line)
    }

    /// returns entire line but with the current char red
    /// will return an Error on empty string
    // TODO: patch to work for unicodes
    fn highlight_current_char_in_line(current_line: &str, char_nr: usize) -> Result<String, ()>{
        Ok(format!("{}{}{}",
            current_line.chars().take(char_nr).collect::<String>(),
            current_line.chars().nth(char_nr).ok_or(())?.to_string().red(),
            current_line.chars()
                .skip(char_nr + 1)
                .take(current_line.chars().count() - char_nr)
                .collect::<String>(),
        ))
    }

    pub fn traceback(program: &str, program_counter: usize, error_msg: &str) -> Result<String, ()>{
        let (line_nr, char_nr, current_line) = Traceback::current_line(program, program_counter);
        let highlighted_current_line = Traceback::highlight_current_char_in_line(current_line, char_nr)?;

        Ok(format!(
            "{}\non line {}, char {}:\n{}\n",
            error_msg, line_nr, char_nr, highlighted_current_line
        ))
    }
}

#[cfg(test)]
mod test_traceback;
