use colored::*;

pub struct Traceback;

impl Traceback{
    /// returns info about current line as a tuple:
    /// (line_nr, char_nr, current_line)
    fn current_line<'a>(program: &'a str, program_counter: usize) -> (usize, usize, &'a str){
        let line_nr = &program[0..program_counter].lines().count();
        let chars_before: &usize = &program.lines()
            .take(*line_nr)
            .fold(0, |sum, l| sum + l.chars().count());
        println!("chars_before = {}, line_nr = {}", chars_before, line_nr);
        println!("pc = {}", program_counter);

        let char_nr = program_counter - chars_before;
        let current_line = &program.lines().nth(line_nr - 1).unwrap();
        println!("current line = {}", current_line);

        (*line_nr, char_nr, current_line)
    }

    /// returns entire line but with the current char red
    fn highlight_current_char_in_line(current_line: &str, char_nr: usize) -> String{
        format!("{}{}{}",
            current_line.chars().take(char_nr).collect::<String>(),
            current_line.chars().nth(char_nr).unwrap().to_string().red(),
            current_line.chars()
                .skip(char_nr + 1)
                .take(current_line.chars().count() - char_nr)
                .collect::<String>(),
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

#[cfg(test)]
mod test_traceback;
