use self::ItemType::*;
use std::io;
use std::io::Write;
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

impl GopherRequest {
    /// Read a Gopher request from a buffer containing a line *without* the trailing CRLF.
    pub fn decode(mut line: EasyBuf) -> Self {
        // Split on TAB if present.
        let query = match line.as_slice().iter().position(|b| *b == b'\t') {
            Some(i) => {
                let mut query = line.split_off(i);
                query.drain_to(1); // Consume the TAB.
                Some(GopherStr::new(query))
            }
            None => None
        };

        GopherRequest {
            selector: GopherStr::new(line),
            query: query,
        }
    }
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

impl GopherResponse {
    /// Encode the response into bytes for sending over the wire.
    pub fn encode<W>(&self, mut buf: W) -> io::Result<()> 
        where W: Write
    {
        match *self {
            GopherResponse::BinaryFile(ref file) => {
                buf.write_all(file.as_slice())?;
            }
            GopherResponse::TextFile(ref file) => {
                // TODO: Escape lines beginning with periods by adding an extra period.
                buf.write_all(file.as_slice())?;
                buf.write_all(b".\r\n")?;
            }
            GopherResponse::Menu(ref entities) => {
                for entity in entities {
                    write!(buf, "{}{}\t{}\t{}\t{}\r\n",
                           entity.item_type.encode(),
                           entity.name,
                           entity.selector,
                           entity.host,
                           entity.port)?;
                }
                buf.write_all(b".\r\n")?;
            }
        }
        Ok(())
    }
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
