use std::collections::HashMap;
use std::path::Path;
use std::process;

extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

// Docopt usage string.
static USAGE: &'static str = "
Usage: sits <dir>
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_dir: String,
}

extern crate sits;

use sits::{Property, PropertyBag, read_property_file};

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let game_props = {
        let path = Path::new(&args.arg_dir).join("Game.txt");
        match read_property_file(path.as_path()) {
            Ok(v) => v,
            Err(e) => {
                println!("Cannot read {:?}: {}", path, e);
                process::exit(1);
            }
        }
    };
    if let Some(&Property::Integer(v)) = game_props.get("Emeralds") {
        println!("Emeralds: {}", v);
    }

    //let party_props: HashMap<&str, PropertyBag> = HashMap::new();
    if let Some(&Property::String(ref v)) = game_props.get("PartyIDs") {
        for id in v.split(",").filter(|&id| id != "0") {
            let path = Path::new(&args.arg_dir).join("Party".to_string() + id + ".txt");
            let party = match read_property_file(path.as_path()) {
                Ok(v) => v,
                Err(e) => {
                    println!("Cannot read {:?}: {}", path, e);
                    process::exit(1);
                }
            };
            if let Some(&Property::String(ref v)) = party.get("Name") {
                println!("{}({}):", v, party.get("Level").unwrap());
                println!("    Int: {}", party.get("Int").unwrap());
                println!("    Dex: {}", party.get("Dex").unwrap());
                println!("    Str: {}", party.get("Str").unwrap());
                println!("    Occ: {}", party.get("Occ").unwrap());
                println!("    Per: {}", party.get("Per").unwrap());
            }
        }
    }
}
