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

pub trait ReadVariableExt: Read {

  /// Reads a variable-length encoded integer.
  /// @see https://msdn.microsoft.com/en-us/library/system.io.binarywriter.write7bitencodedint.aspx
  #[inline]
  fn read_variable_uint(&mut self) -> Result<u64> {
    let mut val = 0u64;
    let mut nread = 0usize;
    loop {
      let byte = try!(self.read_u8());
      val = val | ((byte as u64 & 0x7f) << (nread * 7));
      nread += 1;
      if byte & 0x80 == 0 {
        break;
      }
    }
    Ok(val)
  }

  /// Reads a variable-length encoded integer, representing the length of the string; then
  /// reads that many bytes from the undelying reader and interprets them as a utf8 string.
  #[inline]
  fn read_variable_string(&mut self) -> Result<String> {
    let len = try!(self.read_variable_uint()) as usize;
    let mut buf = vec![0; len];
    let mut nread = 0usize;
    while nread < buf.len() {
      match self.read(&mut buf[nread..]) {
        Ok(0) => return Err(Error::UnexpectedEOF),
        Ok(n) => nread += n,
        Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {},
        Err(e) => return Err(From::from(e))
      }
    }
    match String::from_utf8(buf) {
      Ok(v) => Ok(v),
      Err(e) => Err(Error::from(e)),
    }
  }
}

/// All types that implement `Read` get methods defined in `ReadVariableExt`.
impl<R: Read + ?Sized> ReadVariableExt for R {}
