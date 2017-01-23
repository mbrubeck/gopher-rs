use std::fmt::{self, Display, Write};
use tokio_core::io::EasyBuf;

/// A string of bytes as sent over the wire.
///
/// The contents are assumed to be encoded in ISO-8859-1 (Latin-1).
pub struct GopherStr {
    buf: EasyBuf
}

impl GopherStr {
    /// Create a GopherStr from an EasyBuf.
    pub fn new(buf: EasyBuf) -> Self {
        GopherStr { buf: buf }
    }

    /// Unwrap the inner EasyBuf.
    pub fn into_buf(self) -> EasyBuf {
        self.buf
    }
}

impl AsRef<[u8]> for GopherStr {
    fn as_ref(&self) -> &[u8] {
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
