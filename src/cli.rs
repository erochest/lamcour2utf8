use super::Result;
use lamcour::translate_bytes;
use std::io::{self, Read, Write};

pub fn run() -> Result<()> {
    let mut buffer = Vec::new();
    io::stdin()
        .read_to_end(&mut buffer)
        .map_err(|err| format_err!("Error reading from STDIN: {:?}", &err))?;

    let contents = translate_bytes(buffer.as_slice());

    io::stdout()
        .write_all(contents.as_bytes())
        .map_err(|err| format_err!("Error writing to STDOUT: {:?}", &err))
}
