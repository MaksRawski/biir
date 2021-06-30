#[derive(Debug, PartialEq)]
pub enum Error{
    Syntax(String),
    Runtime(String),
}
