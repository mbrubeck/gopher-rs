use std::io;
use std::io::Write;
use str::GopherStr;
use tokio_core::io::{Codec, EasyBuf};
use types::{GopherRequest, GopherResponse};
use types::GopherResponse::*;

pub struct Server;

impl Codec for Server {
    type In = GopherRequest;
    type Out = GopherResponse;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<GopherRequest>> {
        // Read a CR+LF delimited line.
        let mut line = match buf.as_slice().windows(2).position(|w| w == b"\r\n") {
            Some(i) => buf.drain_to(i),
            None => return Ok(None)
        };
        // Discard the CR+LF.
        buf.drain_to(2);

        // Split on TAB if present.
        let query = match line.as_slice().iter().position(|b| *b == b'\t') {
            Some(i) => {
                let mut query = line.split_off(i);
                query.drain_to(1); // Consume the TAB.
                Some(GopherStr::new(query))
            }
            None => None
        };

        Ok(Some(GopherRequest {
            selector: GopherStr::new(line),
            query: query,
        }))
    }

    fn encode(&mut self, msg: GopherResponse, buf: &mut Vec<u8>) -> io::Result<()> {
        match msg {
            BinaryFile(file) => {
                buf.extend(file.as_slice());
            }
            TextFile(file) => {
                // TODO: Escape lines beginning with periods by adding an extra period.
                buf.extend(file.as_slice());
                buf.extend(b".\r\n");
            }
            Menu(entities) => {
                for entity in entities {
                    write!(buf, "{}{}\t{}\t{}\t{}\r\n",
                           entity.item_type.encode(),
                           entity.name,
                           entity.selector,
                           entity.host,
                           entity.port)?;
                }
                buf.extend(b".\r\n");
            }
        }
        Ok(())
    }
}
