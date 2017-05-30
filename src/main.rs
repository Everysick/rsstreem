extern crate clap;
extern crate rsstreem;
extern crate rsstreem-parser as parser;

use clap::{Arg, App, SubCommand};

use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;

const VERSION_MAJOR: u32 = 0;
const VERSION_MINOR: u32 = 1;

fn main() {
    let matches = App::new("rsstreem")
        .version(&format!("v{}.{}", VERSION_MAJOR, VERSION_MINOR))
        .author("everysick <s.wakeup31@gmail.com>")
        .args_from_usage(
            "<INPUT> 'Source code file to compile'
             -d, --debug 'Display debugging information'")
        .get_matches();

    let path = matches.value_of("INPUT")
        .map(PathBuf::from)
        .unwrap();

    let code = File::open(&path)
        .map_err(|error    | String::from(error.description()) )
        .and_then(|mut file| {
            let mut s = String::new();
            file.read_to_string(&mut s)
                .map_err(|error| String::from(error.description()) )
                .map(|_| s)
        })
        .unwrap();
    
    let ast = parser::parser_code(code.as_ref()).unwrap();
    for node in ast { println!("{}", (*node).fmt_string()) }
}
