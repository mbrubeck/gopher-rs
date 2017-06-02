use bytes::Bytes;
use std::fmt::{self, Debug, Display, Write};
use std::ops::Deref;

/// A string of bytes as sent over the wire.
///
/// The contents are assumed to be encoded in ISO-8859-1 (Latin-1).
#[derive(Clone)]
pub struct GopherStr {
    buf: Bytes
}

impl GopherStr {
    /// Create a GopherStr from a Bytes.
    pub fn new(buf: Bytes) -> Self {
        GopherStr { buf: buf }
    }

    pub fn from_latin1(bytes: &[u8]) -> Self {
        GopherStr { buf: Bytes::from(bytes) }
    }

    /// Unwrap the inner Bytes.
    pub fn into_buf(self) -> Bytes {
        self.buf
    }
}

impl Deref for GopherStr {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.buf
    }
}

impl Display for GopherStr {
    /// Decode from Latin-1 without allocating.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for b in &self.buf {
            f.write_char(b as char)?;
        }
        Ok(())
    }
}

impl Debug for GopherStr {
    /// Decode from Latin-1 without allocating.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char('"')?;
        for b in &self.buf {
            for c in (b as char).escape_default() {
                f.write_char(c)?;
            }
        }
        f.write_char('"')?;
        Ok(())
    }
}
