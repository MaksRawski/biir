use colored::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    Syntax(String),
    Runtime(String),
}

impl Error {
    pub fn msg(self) -> String {
        match self {
            Error::Syntax(msg) => format!("{} {}", "Syntax error:".red(), msg.normal()),
            Error::Runtime(msg) => format!("{} {}", "Runtime error:".red(), msg.normal()),
        }
    }
}
