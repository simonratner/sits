use std::collections;
use std::fmt;

#[derive(Debug)]
pub enum Property {
    Boolean(bool),
    Integer(u32),
    Numeric(f32),
    String(String),
    Unknown(Vec<u8>),
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Property::Boolean(v) => write!(f, "{}", v),
            Property::Integer(v) => write!(f, "{}", v),
            Property::Numeric(v) => write!(f, "{}", v),
            Property::String(ref v) => write!(f, "{}", v),
            Property::Unknown(ref v) => write!(f, "<{} bytes>", v.len()),
        }
    }
}

pub type PropertyBag = collections::HashMap<String, Property>;
