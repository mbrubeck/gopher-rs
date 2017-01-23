use std::io;
use tokio_core::io::{Codec, EasyBuf};
use types::{GopherRequest, GopherResponse};

pub struct Server;

impl Codec for Server {
    type In = GopherRequest;
    type Out = GopherResponse;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<GopherRequest>> {
        // Read a CR+LF delimited line.
        let line = match buf.as_slice().windows(2).position(|w| w == b"\r\n") {
            Some(i) => buf.drain_to(i),
            None => return Ok(None)
        };
        // Discard the CR+LF.
        buf.drain_to(2);

        Ok(Some(GopherRequest::decode(line)))
    }

    fn encode(&mut self, _msg: GopherResponse, buf: &mut Vec<u8>) -> io::Result<()> {
        //msg.encode(buf);
        buf.extend(b"\r\n");
        Ok(())
    }
}
