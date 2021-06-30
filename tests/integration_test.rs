extern crate biir;

use std::fs;
use test_case::test_case;

use biir::parser::Parser;
use biir::utils::Output;

#[cfg(test)]
mod example_programs {
    use super::*;

    // TODO: test programs in programs/
    // they have their expected output given in the first line
    // within square brackets, like this:
    // [expected output]

    #[test]
    fn test_programs() {
        for f in fs::read_dir("./tests/programs").unwrap() {
            let f = f.unwrap();
            let program = fs::read_to_string(f.path()).unwrap();
            let filename = f.file_name().into_string().unwrap();

            test_program(&program, filename);
        }
    }

    fn test_program(program: &str, name: String) {
        dbg!(name);
        let mut parser = Parser::new();
        parser.output = Output::Vector(Vec::new());

        // expected output is in the first line
        let expected_output = program.lines().next().unwrap().replace("\\n", "\n");

        assert_eq!(parser.execute(&program, false, false), Ok(()));
        assert_eq!(format!("[{}]", parser.output.read()), expected_output);
    }

    // test program with newlines
    // test program with windows style (\r\n) newlines
}
