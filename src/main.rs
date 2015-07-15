use std::cell::RefCell;
use std::path::Path;
use std::process;
use std::rc::Rc;

extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

// Docopt usage string.
static USAGE: &'static str = r#"
Usage: sits <dir>
"#;

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_dir: String,
}

extern crate sits;
use sits::{Property, PropertyMap, PropertyMapRef, read_property_file, run_ui_loop};

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let game: PropertyMapRef = Rc::new(RefCell::new({
        let path = Path::new(&args.arg_dir).join("Game.txt");
        match read_property_file(path.as_path()) {
            Ok(v) => v,
            Err(e) => {
                println!("Cannot read {:?}: {}", path, e);
                process::exit(1);
            }
        }
    }));

    let mut party: Vec<PropertyMapRef> = Vec::new();
    if let Some(&Property::String(ref v)) = game.borrow().get("PartyIDs") {
        for id in v.split(",") {
            let path = Path::new(&args.arg_dir).join("Party".to_string() + id + ".txt");
            let party_member = match read_property_file(path.as_path()) {
                Ok(v) => v,
                Err(e) => {
                    println!("Cannot read {:?}: {}", path, e);
                    process::exit(1);
                }
            };
            party.push(Rc::new(RefCell::new(party_member)));
        }
    }

    run_ui_loop(game, &party);
}
