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

