use std::collections::HashMap;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub enum Property {
    Boolean(bool),
    Integer(u32),
    Float(f32),
    String(String),
    Unknown(Vec<u8>),
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
            Property::Unknown(ref v) => write!(f, "<{} bytes>", v.len()),
        }
    }
}

/// A collection of properties.
pub type PropertyMap = HashMap<String, Property>;

/// A refcounted collection of properties.
///
/// Since we need to share mutable state with 'static ui callbacks,
/// we clone a refcounted cell for moving into each callback.
pub type PropertyMapRef = Rc<RefCell<PropertyMap>>;
