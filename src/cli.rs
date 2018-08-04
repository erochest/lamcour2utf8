use super::Result;
use lamcour::translate_bytes;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub fn run() -> Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.len() == 0 {
        process_stdin()
    } else {
        process_file(&PathBuf::from(&args[0]))
    }
}

fn process_stdin() -> Result<()> {
    process(&mut io::stdin(), &mut io::stdout())
}

fn process_file(path: &Path) -> Result<()> {
    let mut input =
        File::open(path).map_err(|err| format_err!("Unable to open file {:?}: {:?}", path, &err))?;

    let output_path = path.with_extension("utf8");
    let mut output = File::create(&output_path)
        .map_err(|err| format_err!("Unable to create file {:?}: {:?}", &output_path, &err))?;

    process(&mut input, &mut output)
}

fn process<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<()> {
    let mut buffer = Vec::new();
    input
        .read_to_end(&mut buffer)
        .map_err(|err| format_err!("Error reading from STDIN: {:?}", &err))?;

    let contents = translate_bytes(buffer.as_slice());

    output
        .write_all(contents.as_bytes())
        .map_err(|err| format_err!("Error writing to STDOUT: {:?}", &err))
}
