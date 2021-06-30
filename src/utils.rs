use std::io;
use std::io::{Read, Write};
use termion::raw::IntoRawMode;

pub fn getchar() -> Result<u8, String>{
    let mut buffer = [0];
    let stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = io::stdin();

    stdout.lock().flush().unwrap();

    // TODO: this looks like it could be done more concisely
    return if let Ok(_) = stdin.read_exact(&mut buffer){
        if buffer[0] == 3{
            Err( String::from("got Ctrl-C, Exiting!") )
        }
        else{
            Ok( buffer[0] )
        }
    }
    else{
        Err( String::from("not enough input given") )
    }
}

/// common interface for writing to stdout
/// as well as Vector, to allow integration tests
pub enum Output{
    Stdout(std::io::Stdout),
    Vector(Vec<String>),
}

impl Output{
    pub fn write(&mut self, msg: String){
        match self{
            Output::Stdout(out) => { out.write(msg.as_bytes()).unwrap(); },
            Output::Vector(out) => { out.push(msg) },
        };
    }
    /// return string of everyting that has been written
    /// ```
    /// use biir::utils::Output;
    ///
    /// let mut output = Output::Vector(Vec::new());
    ///
    /// output.write("Hello, ".to_string());
    /// output.write("world!".to_string());
    ///
    /// assert_eq!(output.read(), "Hello, world!");
    /// ```
    /// panic when used with Output::Stdout
    /// ```
    /// use biir::utils::Output;
    ///
    /// let mut output = Output::Stdout(std::io::stdout());
    /// let result = std::panic::catch_unwind(|| output.read());
    ///
    /// assert!(result.is_err());
    /// ```
    pub fn read(&self) -> String{
        match self{
            Output::Stdout(_) => panic!(),
            Output::Vector(v) => v.join(""),
        }
    }
}
