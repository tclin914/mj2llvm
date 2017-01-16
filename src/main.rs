extern crate rustc_serialize;
extern crate docopt;

extern crate regex;

mod lexer;

use docopt::Docopt;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use lexer::*;

const USAGE: &'static str = "
Usage: 
    mj2llvm [-l | -p | -i] [--verbose] <source>

Options:
    -l  Run only lexer and show its output.
    -p  Run only parser and show its output.
    -i  Run only IR builder and show its output.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_l: bool,
    flag_p: bool,
    flag_i: bool,
    flag_verbose: bool,
    arg_source: String
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let mut file = match File::open(&args.arg_source) {
        Err(why) => panic!("couldn't open {}: {}", args.arg_source, 
                          why.description()),
        Ok(file) => {
            if args.flag_verbose {
                println!("Open the file: {}", args.arg_source); 
            }
            file
        },
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", args.arg_source,
                          why.description()),
        Ok(_) => {
            if args.flag_verbose {
                println!("Read the file: {}", args.arg_source);
            }
        },
    }
}
