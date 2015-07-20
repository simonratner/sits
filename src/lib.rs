#[macro_use]
extern crate byteorder;
extern crate iup;
extern crate time;
extern crate xml;

mod io;
mod parser;
mod property;

pub use ui::{ui_loop};

mod ui;
