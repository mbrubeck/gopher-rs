use self::ItemType::*;
use std::io;
use std::io::Write;
use str::GopherStr;
use tokio_core::io::EasyBuf;

pub enum Void {}

/// A client-to-server message.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub enum GopherResponse {
    /// A list of resources.
    Menu(Vec<DirEntity>),

    /// A text document.
    TextFile(EasyBuf),

    /// A binary file download.
    BinaryFile(EasyBuf),

    /// A single menu item enclosed in a Gopher+ protocol response.
    ///
    /// Useful for redirecting Gopher+ clients to the standard Gopher protocol.
    GopherPlusRedirect(DirEntity),
}

impl GopherResponse {
    /// Construct a menu with a single error line.
    pub fn error(text: GopherStr) -> Self {
        GopherResponse::Menu(vec![
            DirEntity {
                item_type: Error,
                name: text,
                selector: GopherStr::from_latin1(b"error"),
                host: GopherStr::from_latin1(b"error.host"),
                port: 0,
            }
        ])
    }

    /// Encode the response into bytes for sending over the wire.
    pub fn encode<W>(&self, mut buf: &mut W) -> io::Result<()>
        where W: Write
    {
        match *self {
            GopherResponse::BinaryFile(ref file) => {
                buf.write_all(file.as_slice())?;
            }
            GopherResponse::TextFile(ref file) => {
                // TODO: Escape lines beginning with periods by adding an extra period.
                buf.write_all(file.as_slice())?;
                buf.write_all(b"\r\n.\r\n")?;
            }
            GopherResponse::Menu(ref entities) => {
                for entity in entities {
                    entity.encode(buf)?;
                }
                buf.write_all(b".\r\n")?;
            }
            GopherResponse::GopherPlusRedirect(ref entity) => {
                buf.write_all(b"+-1\r\n+INFO: ")?;
                entity.encode(buf)?;
            }
        }
        Ok(())
    }
}

/// A list of Gopher resources.
pub struct Menu {
    pub entities: Vec<DirEntity>,
}

/// An menu item in a directory of Gopher resources.
#[derive(Clone, Debug)]
pub struct DirEntity {
    /// The type of the resource
    pub item_type: ItemType,
    /// String to display to the user.
    pub name: GopherStr,
    /// Path or identifier used for requesting this resource.
    pub selector: GopherStr,
    /// The hostname of the server hosting this resource.
    pub host: GopherStr,
    /// The TCP port of the server hosting this resource.
    pub port: u16,
}

impl DirEntity {
    pub fn encode<W>(&self, mut buf: &mut W) -> io::Result<()>
        where W: Write
    {
        buf.write_all(&[self.item_type.encode()])?;
        buf.write_all(&self.name)?;
        buf.write_all(b"\t")?;
        buf.write_all(&self.selector)?;
        buf.write_all(b"\t")?;
        buf.write_all(&self.host)?;
        write!(buf, "\t{}\r\n", self.port)?;
        Ok(())
    }
}

/// The type of a resource in a Gopher directory.
///
/// For more details, see: https://tools.ietf.org/html/rfc1436
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
    /// Item is a non-standard type
    Other(u8),
}

impl ItemType {
    pub fn decode(b: u8) -> Self {
        match b {
            b'0' => File,
            b'1' => Dir,
            b'2' => CsoServer,
            b'3' => Error,
            b'4' => BinHex,
            b'5' => Dos,
            b'6' => Uuencoded,
            b'7' => IndexServer,
            b'8' => Telnet,
            b'9' => Binary,
            b'+' => RedundantServer,
            b'T' => Tn3270,
            b'g' => Gif,
            b'I' => Image,
            byte => Other(byte)
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
            Other(byte)     => byte,
        }
    }
}
