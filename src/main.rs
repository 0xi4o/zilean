#![allow(dead_code)]

use std::io;

use args::parse_args;

mod args;
// mod utils;

fn main() -> Result<(), io::Error> {
    let _ = parse_args();
    Ok(())
}
