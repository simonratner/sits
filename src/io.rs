use std::fmt;
use std::io::{self, Read};
use std::result;
use std::string;

use byteorder::{self, ReadBytesExt};

/// A short-hand for `result::Result<T, io::Error>`.
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Io(io::Error),
  Utf8(string::FromUtf8Error),
  UnexpectedTag(u8),
  UnexpectedEOF,
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Error { Error::Io(err) }
}

impl From<string::FromUtf8Error> for Error {
  fn from(err: string::FromUtf8Error) -> Error { Error::Utf8(err) }
}

impl From<byteorder::Error> for Error {
  fn from(err: byteorder::Error) -> Error {
    match err {
      byteorder::Error::Io(err) => Error::Io(err),
      byteorder::Error::UnexpectedEOF => Error::UnexpectedEOF,
    }
  }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnexpectedTag(v) => write!(f, "Unexpected tag 0x{:x}.", v),
            Error::UnexpectedEOF => write!(f, "Unexpected end of file."),
            Error::Utf8(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
        }
    }
}

pub trait ReadSizedStringExt: Read {
  /// Reads an unsigned 8 bit integer from the underlying reader, then
  /// reads that many bytes and interprets them as a utf8 string.
  #[inline]
  fn read_sized_string(&mut self) -> Result<String> {
    let mut len = try!(self.read_u8()) as usize;
    let mut buf = vec![0; len];
    match self.read(&mut buf) {
      Ok(nread) if (nread == len) => {
        match String::from_utf8(buf) {
          Ok(v) => Ok(v),
          Err(e) => Err(Error::from(e)),
        }
      },
      Ok(..) => Err(Error::UnexpectedEOF),
      Err(e) => Err(Error::from(e)),
    }
  }
}

/// All types that implement `Read` get methods defined in `ReadSizedStringExt`.
impl<R: Read + ?Sized> ReadSizedStringExt for R {}
