use bytes::{BytesMut, BufMut};
use std::io;
use tokio_io::codec::{Decoder, Encoder};
use types::{GopherRequest, GopherResponse};

/// A codec for building a Gopher server.
pub struct ServerCodec;

impl Decoder for ServerCodec {
    type Item = GopherRequest;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        // Read a CR+LF delimited line.
        let line = match buf.windows(2).position(|w| w == b"\r\n") {
            Some(i) => buf.split_to(i),
            None => return Ok(None)
        };
        // Discard the CR+LF.
        buf.split_to(2);

        Ok(Some(GopherRequest::decode(line.freeze())))
    }
}

impl Encoder for ServerCodec {
    type Item = GopherResponse;
    type Error = io::Error;

    fn encode(&mut self, message: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        message.encode(buf.writer())
    }
}
