extern crate biir;
use biir::parser::Parser;

#[cfg(test)]
mod test_parser{
    use super::*;
    #[test]
    fn test_closing_bracket_finder(){
        let parser = Parser::new();
        assert_eq!( parser.find_closing_bracket(0, "[]"), Ok(1) );
        assert_eq!( parser.find_closing_bracket(1, "[]"), Err(()) );
        assert_eq!( parser.find_closing_bracket(1, "[[[]]]"), Ok(4) );
        assert_eq!( parser.find_closing_bracket(0, "[[][]]"), Ok(5) );
    }
}

mod integration_tests{
    use super::*;
    fn hello_world(){
        let parser = Parser::new();
    }
}
