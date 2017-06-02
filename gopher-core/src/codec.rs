use bytes::{Bytes, BytesMut, BufMut};
use std::io;
use tokio_io::codec::{Decoder, Encoder};
use tokio_proto::streaming::pipeline::Frame;
use types::{GopherRequest, GopherResponse, Void};

/// A codec for building a Gopher server.
pub struct ServerCodec;

impl Decoder for ServerCodec {
    type Item = Frame<GopherRequest, Void, io::Error>;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        // Read a CR+LF delimited line.
        let line = match buf.windows(2).position(|w| w == b"\r\n") {
            Some(i) => buf.split_to(i),
            None => return Ok(None)
        };
        // Discard the CR+LF.
        buf.split_to(2);

        Ok(Some(Frame::Message {
            message: GopherRequest::decode(line.freeze()),
            body: false,
        }))
    }
}

impl Encoder for ServerCodec {
    type Item = Frame<GopherResponse, Bytes, io::Error>;
    type Error = io::Error;

    fn encode(&mut self, frame: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        match frame {
            Frame::Message { message, .. } => {
                message.encode(buf.writer())
            }
            Frame::Body { chunk } => {
                if let Some(chunk) = chunk {
                    buf.extend(&chunk);
                }
                Ok(())
            }
            Frame::Error { error } => {
                Err(error)
            }
        }
    }
}
