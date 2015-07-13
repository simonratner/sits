use std::collections;

#[derive(Debug)]
pub enum Property {
    Boolean(bool),
    Integer(u32),
    Numeric(f32),
    String(String),
    Unknown(Vec<u8>),
}

pub type PropertyBag = collections::HashMap<String, Property>;
