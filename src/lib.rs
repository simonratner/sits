#[macro_use]
extern crate iup;
extern crate time;
extern crate byteorder;

mod io;
mod parser;
mod property;

pub use ui::{ui_loop};

mod ui;
