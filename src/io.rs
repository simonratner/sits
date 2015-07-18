use std::fmt;
use std::io::{self, Read, Write};
use std::result;
use std::string;

use byteorder::{self, ReadBytesExt, WriteBytesExt};

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

/// Extension for reading length-prefixed strings.
pub trait ReadVariableExt: Read {
    /// Reads a variable-length encoded 32-bit integer.
    /// https://msdn.microsoft.com/en-us/library/system.io.binarywriter.write7bitencodedint.aspx
    fn read_variable_uint(&mut self) -> Result<u32> {
        let mut val = 0u32;
        let mut nread = 0usize;
        while nread < 5 {
            let byte = try!(self.read_u8());
            val = val | (((byte & 0x7f) as u32) << (nread * 7));
            nread += 1;
            if byte & 0x80 == 0 {
                break;
            }
        }
        Ok(val)
    }

    /// Reads a variable-length encoded integer, representing the length of the string; then
    /// reads that many bytes from the underlying reader and interprets them as a utf8 string.
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
        String::from_utf8(buf).map_err(Error::from)
    }
}

/// All types that implement `Read` get methods defined in `ReadVariableExt`.
impl<R: Read + ?Sized> ReadVariableExt for R {}

/// Extension for writing length-prefixed strings.
pub trait WriteVariableExt: Write {
    /// Writes a variable-length encoded 32-bit integer.
    /// https://msdn.microsoft.com/en-us/library/system.io.binarywriter.write7bitencodedint.aspx
    fn write_variable_uint(&mut self, n: u32) -> Result<()> {
        let mut val = n;
        while val > 0x7f {
            try!(self.write_u8(((val & 0x7f) | 0x80) as u8));
            val = val >> 7;
        }
        try!(self.write_u8((val & 0x7f) as u8));
        Ok(())
    }

    /// Writes a variable-length encoded integer, representing the length of the string; then
    /// writes that many bytes to the underlying writer, representing the utf8-encoded string.
    fn write_variable_string(&mut self, s: &str) -> Result<()> {
        try!(self.write_variable_uint(s.len() as u32));
        try!(self.write_all(s.as_bytes()));
        Ok(())
    }
}

/// All types that implement `Write` get methods defined in `WriteVariableExt`.
impl<W: Write + ?Sized> WriteVariableExt for W {}
