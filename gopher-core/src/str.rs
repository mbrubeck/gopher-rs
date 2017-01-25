use std::fmt::{self, Debug, Display, Write};
use std::ops::Deref;
use tokio_core::io::EasyBuf;

/// A string of bytes as sent over the wire.
///
/// The contents are assumed to be encoded in ISO-8859-1 (Latin-1).
#[derive(Clone)]
pub struct GopherStr {
    buf: EasyBuf
}

impl GopherStr {
    /// Create a GopherStr from an EasyBuf.
    pub fn new(buf: EasyBuf) -> Self {
        GopherStr { buf: buf }
    }

    pub fn from_latin1(bytes: &[u8]) -> Self {
        let mut buf = EasyBuf::new();
        buf.get_mut().extend(bytes);
        GopherStr { buf: buf }
    }

    /// Unwrap the inner EasyBuf.
    pub fn into_buf(self) -> EasyBuf {
        self.buf
    }
}

impl Deref for GopherStr {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.buf.as_slice()
    }
}

impl Display for GopherStr {
    /// Decode from Latin-1 without allocating.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for b in self.buf.as_slice() {
            f.write_char(*b as char)?;
        }
        Ok(())
    }
}

impl Debug for GopherStr {
    /// Decode from Latin-1 without allocating.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char('"')?;
        for b in self.buf.as_slice() {
            for c in (*b as char).escape_default() {
                f.write_char(c)?;
            }
        }
        f.write_char('"')?;
        Ok(())
    }
}
