#[cfg(test)]
extern crate biir;

// #[test_case( "!TAPE\n+!TAPE\n+!TAPE",
//     r"(?m).*!TAPE.*: \[0\] \n.*!TAPE.*: \[1\] \n.*!TAPE.*: \[2\]" ; "!TAPE")]
// fn test_output(program: &str, expected_output: &str) {
//     let mut interpreter = Interpreter::new(Modes {
//         debug: true,
//         numerical: false,
//         big_int: false,
//     });
//     interpreter.output = Output::Vector(Vec::new());

//     let expected_output = Regex::new(expected_output).unwrap();

//     assert_eq!(interpreter.execute(program), Ok(()));
//     assert!(expected_output.is_match(&interpreter.output.read()));
// }

// #[test_case("[", r"Syntax error.*\n.*char 1:\n"; "Syntax error")]
// #[test_case("<", r"Runtime error.*\n.*char 1:\n"; "Runtime error")]
// #[test_case("😎[", r".*Syntax error.*\n.*char 2.*"; "Syntax error with unicodes")]
// #[test_case("😎<", r"Runtime error.*\n.*char 2:\n"; "Runtime error with unicodes")]
// fn test_error_messages(program: &str, expected_error_message: &str) {
//     let mut interpreter = Interpreter::new(Modes {
//         debug: true,
//         numerical: false,
//         big_int: false,
//     });
//     interpreter.output = Output::Vector(Vec::new());

//     let expected_error_message = Regex::new(expected_error_message).unwrap();
//     let error_msg = interpreter.execute(program).err().unwrap();

//     assert!(expected_error_message.is_match(&error_msg));
// }

// #[test]
// fn test_example_programs() {
//     for f in fs::read_dir("./tests/programs").unwrap() {
//         let f = f.unwrap();
//         let program = f.path();
//         test_program(program);
//     }
// }

// fn test_program(program: std::path::PathBuf) {
//     dbg!(program.file_name().unwrap());
//     let mut interpreter = Interpreter::new(Modes {
//         debug: true,
//         numerical: false,
//         big_int: false,
//     });
//     interpreter.output = Output::Vector(Vec::new());

//     // expected output is in the first line
//     let expected_output = fs::read_to_string(&program)
//         .unwrap()
//         .lines()
//         .next()
//         .unwrap()
//         .replace("\\n", "\n");

//     assert_eq!(interpreter.run(&program), Ok(()));
//     assert_eq!(format!("[{}]", interpreter.output.read()), expected_output);
// }

// TODO: test program with newlines
// TODO: test program with windows style (\r\n) newlines
// TODO: test numerical mode (with big ints)
