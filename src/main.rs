use std::path::Path;
use std::process;

extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

// Docopt usage string.
static USAGE: &'static str = "
Usage: sits <source>
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_source: String,
}

extern crate sits;

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    match sits::read_property_file(Path::new(&args.arg_source)) {
        Ok(v) => v,
        Err(e) => {
            println!("Cannot read '{}': {}", args.arg_source, e);
            process::exit(1);
        }
    };
}
