use std::io;
use std::io::{Read, Write};
use termion::raw::IntoRawMode;

pub fn getchar() -> u8{
    let mut buffer = [0];
    let stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = io::stdin();

    stdout.lock().flush().unwrap();
    stdin.read_exact(&mut buffer).unwrap();

    buffer[0]
}

