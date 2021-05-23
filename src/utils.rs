use std::io;
use std::io::{Read, Write};
use termion::raw::IntoRawMode;

pub fn getchar() -> Result<u8, ()>{
    let mut buffer = [0];
    let stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = io::stdin();

    stdout.lock().flush().unwrap();

    // TODO: this looks like it could be done more concisely
    return if let Ok(_) = stdin.read_exact(&mut buffer){
        Ok( buffer[0] )
    }
    else{
        Err( () )
    }
}

