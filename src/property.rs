use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum Property {
    Boolean(bool),
    Integer(u32),
    Float(f32),
    String(String),
    List(Vec<String>),
    Unknown(Vec<u8>, u8),
}

impl From<bool> for Property {
    fn from(v: bool) -> Self { Property::Boolean(v) }
}

impl From<u32> for Property {
    fn from(v: u32) -> Self { Property::Integer(v) }
}

impl From<f32> for Property {
    fn from(v: f32) -> Self { Property::Float(v) }
}

impl From<String> for Property {
    fn from(v: String) -> Self { Property::String(v) }
}

impl From<Vec<String>> for Property {
    fn from(v: Vec<String>) -> Self { Property::List(v) }
}

impl<'a> From<&'a str> for Property {
    fn from(v: &'a str) -> Self { Property::String(String::from(v)) }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Property::Boolean(v) => write!(f, "{}", v),
            Property::Integer(v) => write!(f, "{}", v),
            Property::Float(v) => write!(f, "{}", v),
            Property::String(ref v) => write!(f, "{}", v),
            Property::List(ref v) => {
                let joined: String = v.iter().fold("".to_string(), |mut i, j| {
                    if !i.is_empty() {
                        i.push_str(",");
                    }
                    i.push_str(j);
                    i
                });
                write!(f, "{}", joined)
            },
            Property::Unknown(ref v, _) => write!(f, "<{} bytes>", v.len()),
        }
    }
}

/// A collection of properties.
pub type PropertyMap = HashMap<String, Property>;
