use super::*;

use regex::Regex;
use test_case::test_case;

#[test_case("[[][]]", Ok(vec![0, 5, 1, 2, 3, 4]) )]
#[test_case("[[][]", Err((0, "Opening")))]
#[test_case("[][]]", Err((4, "Closing")))]
fn test_brackets_finder(program: &str, test_argument: Result<Vec<usize>, (ProgramCounter, &str)>) {
    if test_argument.is_ok() {
        let mut expected_brackets = BracketsMap::new();
        let test_argument = test_argument.ok().unwrap();

        // convert argument to proper hashmap
        let keys = test_argument.iter().step_by(2);
        let values = test_argument.iter().skip(1).step_by(2);
        let brackets = keys.zip(values);

        for (k, v) in brackets {
            expected_brackets.insert(*k, *v);
        }

        assert_eq!(Parser::find_brackets(program), Ok(expected_brackets));
    } else {
        let test_argument = test_argument.err().unwrap();
        let regex = Regex::new(&format!(
            "{} bracket doesn't have a matching .* bracket!",
            test_argument.1
        ))
        .unwrap();
        let result = Parser::find_brackets(program).err().unwrap();

        let error_index = result.1;
        let error_msg = result.0;

        assert_eq!(error_index, test_argument.0);

        if let Error::Syntax(v) = error_msg {
            assert!(regex.is_match(&v))
        } else {
            panic!("It should have been a syntax error!");
        }
    };
}
