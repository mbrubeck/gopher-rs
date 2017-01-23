use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_1;
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

    /// Convert this string from Latin-1 to UTF-8.
    pub fn to_string(&self) -> String {
        ISO_8859_1.decode(self.buf.as_slice(), DecoderTrap::Strict)
            .expect("All byte strings are valid Latin-1")
    }
}

impl AsRef<[u8]> for GopherStr {
    fn as_ref(&self) -> &[u8] {
        self.buf.as_slice()
    }
}
