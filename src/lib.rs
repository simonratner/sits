use std::fs::{File};
use std::io::{BufReader, BufRead, Read};
use std::path::Path;

#[macro_use]
extern crate iup;
extern crate byteorder;
use byteorder::{NativeEndian, ReadBytesExt};

pub use prop::{Property, PropertyMap, PropertyMapRef};

pub use io::{Error, Result};
use io::{ReadVariableExt};

pub use ui::{run_ui_loop};

mod prop;
mod io;
mod ui;

pub fn read_property_file(path: &Path) -> Result<PropertyMap> {
    let file = try!(File::open(path));
    let mut buf = BufReader::new(&file);
    let mut res = PropertyMap::new();
    loop {
        let done = try!(match buf.read_u8() {
            Ok(0x7e) => Ok(false),
            Ok(v) => Err(Error::UnexpectedTag(v)),
            Err(byteorder::Error::UnexpectedEOF) => Ok(true),
            Err(e) => Err(Error::from(e)),
        });
        if done {
            break;
        }
        let name = try!(buf.read_variable_string());
        let data_len = try!(buf.read_u32::<NativeEndian>()) as usize;
        if data_len > 0 {
            let data_type = try!(buf.read_u8());
            res.insert(name, match data_type {
                0x01 => Property::String(try!(buf.read_variable_string())),
                0x02 => Property::Integer(try!(buf.read_u32::<NativeEndian>())),
                0x06 => Property::Float(try!(buf.read_f32::<NativeEndian>())),
                0x09 => Property::Boolean(try!(buf.read_u8()) != 0),
                _    => {
                    let mut v = vec![0; data_len - 2];
                    try!(buf.read(&mut v));
                    Property::Unknown(v)
                }
            });
            buf.consume(1);
        }
    }
    Ok(res)
}
