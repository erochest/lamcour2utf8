use super::{recode_direntry, recode_stdin, Result};
use std::env;
use std::path::PathBuf;

// TODO: test
pub fn run() -> Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.len() == 0 {
        recode_stdin()
    } else {
        args.into_iter()
            .try_for_each(|arg| recode_direntry(&PathBuf::from(&arg)))
    }
}
