use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_1;
use self::ItemType::*;

pub enum GopherError {
    UnknownType,
}

pub type GopherResult<T=()> = Result<T, GopherError>;

pub fn from_latin1(buf: &[u8]) -> String {
    ISO_8859_1.decode(buf, DecoderTrap::Strict).unwrap()
}

/// A client-to-server message.
pub struct GopherRequest {
    /// Identifier of the resource to fetch. May be an empty string.
    pub selector: String,
    /// Search string for a full-text search transaction.
    pub query: Option<String>
}

impl GopherRequest {
    pub fn decode(buf: &[u8]) -> Self {
        match buf.iter().position(|b| *b == b'\t') {
            Some(i) => GopherRequest {
                selector: from_latin1(&buf[..i]),
                query: Some(from_latin1(&buf[(i+1)..]))
            },
            None => GopherRequest {
                selector: from_latin1(buf),
                query: None
            }
        }
    }
}

/// A server-to-client message.
pub enum GopherResponse {
    /// A list of resources.
    Menu(Vec<DirEntity>),
    /// A text document.
    Text(String),
    /// A binary file download.
    Binary(Vec<u8>),
}

pub struct Menu {
    pub entities: Vec<DirEntity>,
}

pub struct DirEntity {
    pub item_type: ItemType,
    pub name: String,
    pub selector: String,
    pub host: String,
    pub port: u16,
}

impl DirEntity {
    fn encode(&self, out: &mut Vec<u8>) {
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ItemType {
    /// Item is a file
    File,
    /// Item is a directory
    Dir,
    /// Item is a CSO phone-book server
    CsoServer,
    /// Error
    Error,
    /// Item is a BinHexed Macintosh file.
    BinHex,
    /// Item is DOS binary archive of some sort.
    ///
    /// Client must read until the TCP connection closes.  Beware.
    Dos,
    /// Item is a UNIX uuencoded file.
    Uuencoded,
    /// Item is an Index-Search server.
    IndexServer,
    /// Item points to a text-based telnet session.
    Telnet,
    /// Item is a binary file! Client must read until the TCP connection closes.  Beware
    Binary,
    /// Item is a redundant server
    RedundantServer,
    /// Item points to a text-based tn3270 session.
    Tn3270,
    /// Item is a GIF format graphics file.
    Gif,
    /// Item is some kind of image file.  Client decides how to display.
    Image,
}

impl ItemType {
    pub fn decode(b: u8) -> GopherResult<Self> {
        match b {
            b'0' => Ok(File),
            b'1' => Ok(Dir),
            b'2' => Ok(CsoServer),
            b'3' => Ok(Error),
            b'4' => Ok(BinHex),
            b'5' => Ok(Dos),
            b'6' => Ok(Uuencoded),
            b'7' => Ok(IndexServer),
            b'8' => Ok(Telnet),
            b'9' => Ok(Binary),
            b'+' => Ok(RedundantServer),
            b'T' => Ok(Tn3270),
            b'g' => Ok(Gif),
            b'I' => Ok(Image),
            _ => Err(GopherError::UnknownType),
        }
    }

    pub fn encode(self) -> u8 {
        match self {
            File            => b'0',
            Dir             => b'1',
            CsoServer       => b'2',
            Error           => b'3',
            BinHex          => b'4',
            Dos             => b'5',
            Uuencoded       => b'6',
            IndexServer     => b'7',
            Telnet          => b'8',
            Binary          => b'9',
            RedundantServer => b'+',
            Tn3270          => b'T',
            Gif             => b'g',
            Image           => b'I',
        }
    }
}
