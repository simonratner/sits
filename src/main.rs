extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

// Docopt usage string.
static USAGE: &'static str = r#"
Usage: sits [<dir>]
"#;

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_dir: String,
}

extern crate sits;

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    match sits::ui_loop() {
        Err(e) => println!("{}", e),
        _ => {}
    };
}
