use self::ItemType::*;
use str::GopherStr;
use tokio_core::io::EasyBuf;

pub enum GopherError {
    UnknownType,
}

pub type GopherResult<T=()> = Result<T, GopherError>;

/// A client-to-server message.
pub struct GopherRequest {
    /// Identifier of the resource to fetch. May be an empty string.
    pub selector: GopherStr,
    /// Search string for a full-text search transaction.
    pub query: Option<GopherStr>
}

/// A server-to-client message.
pub enum GopherResponse {
    /// A list of resources.
    Menu(Vec<DirEntity>),
    /// A text document.
    TextFile(EasyBuf),
    /// A binary file download.
    BinaryFile(EasyBuf),
}

pub struct Menu {
    pub entities: Vec<DirEntity>,
}

pub struct DirEntity {
    pub item_type: ItemType,
    pub name: GopherStr,
    pub selector: GopherStr,
    pub host: GopherStr,
    pub port: u16,
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

    pub fn encode(self) -> char {
        match self {
            File            => '0',
            Dir             => '1',
            CsoServer       => '2',
            Error           => '3',
            BinHex          => '4',
            Dos             => '5',
            Uuencoded       => '6',
            IndexServer     => '7',
            Telnet          => '8',
            Binary          => '9',
            RedundantServer => '+',
            Tn3270          => 'T',
            Gif             => 'g',
            Image           => 'I',
        }
    }
}
