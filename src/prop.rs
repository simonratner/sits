use std::collections::HashMap;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

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

/// A collection of properties.
pub type PropertyMap = HashMap<String, Property>;

/// A refcounted collection of properties.
///
/// Since we need to share mutable state with 'static ui callbacks,
/// we clone a refcounted cell for moving into each callback.
pub type PropertyMapRef = Rc<RefCell<PropertyMap>>;
