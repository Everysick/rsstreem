#[macro_use]
extern crate clap;
extern crate rsstreem;
extern crate rsstreem_parser as parser;

use clap::{App, Arg};
use parser::parse;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let matches = App::new("rsstreem-language")
        .version(crate_version!())
        .author("everysick <s.wakeup31@gmail.com>")
        .about("Implementation of streem by Rust.")
        .arg(Arg::with_name("file").short("f").long("file").help(
            "Target file for execute",
        ))
        .get_matches();

    // Some code is not verified yet.
    // TODO: parse from FILE
    if let Some(f) = matches.value_of("file") {
        let path = PathBuf::from(f);

        let code = File::open(&path)
            .map_err(|error| String::from(error.description()))
            .and_then(|mut file| {
                let mut s = String::new();
                file.read_to_string(&mut s)
                    .map_err(|error| String::from(error.description()))
                    .map(|_| s)
            })
            .unwrap();

        let ast = parse::parse_code(code.as_ref()).unwrap();
    }
}
