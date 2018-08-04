#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
extern crate spectral;

use std::result;

pub type Result<R> = result::Result<R, failure::Error>;

pub mod cli;
pub mod lamcour;
