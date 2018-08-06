#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
extern crate spectral;
extern crate walkdir;

use lamcour::translate_bytes;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::result;

pub type Result<R> = result::Result<R, failure::Error>;

pub mod cli;
pub mod lamcour;

pub fn recode_stdin() -> Result<()> {
    recode(&mut io::stdin(), &mut io::stdout())
}

pub fn recode_direntry(path: &Path) -> Result<()> {
    for entry in walkdir::WalkDir::new(path) {
        let entry = entry.map_err(|err| format_err!("Unable to walk {:?}: {:?}", path, &err))?;
        let child = entry.path();
        if child.is_file() {
            recode_file(child)?;
        }
    }

    Ok(())
}

pub fn recode_file(path: &Path) -> Result<()> {
    let mut input =
        File::open(path).map_err(|err| format_err!("Unable to open file {:?}: {:?}", path, &err))?;

    let output_path = path.with_extension("utf8");
    let mut output = File::create(&output_path)
        .map_err(|err| format_err!("Unable to create file {:?}: {:?}", &output_path, &err))?;

    recode(&mut input, &mut output)
}

pub fn recode<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<()> {
    let mut buffer = Vec::new();
    input
        .read_to_end(&mut buffer)
        .map_err(|err| format_err!("Error reading from STDIN: {:?}", &err))?;

    let contents = translate_bytes(buffer.as_slice());

    output
        .write_all(contents.as_bytes())
        .map_err(|err| format_err!("Error writing to STDOUT: {:?}", &err))
}

#[cfg(test)]
mod test;
