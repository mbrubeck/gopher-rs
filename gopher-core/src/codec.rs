use std::io;
use tokio_core::io::{Codec, EasyBuf};
use tokio_proto::streaming::pipeline::Frame;
use types::{GopherRequest, GopherResponse, Void};

/// A codec for building a Gopher server using tokio-core.
pub struct ServerCodec;

impl Codec for ServerCodec {
    type In = Frame<GopherRequest, Void, io::Error>;
    type Out = Frame<GopherResponse, EasyBuf, io::Error>;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        // Read a CR+LF delimited line.
        let line = match buf.as_slice().windows(2).position(|w| w == b"\r\n") {
            Some(i) => buf.drain_to(i),
            None => return Ok(None)
        };
        // Discard the CR+LF.
        buf.drain_to(2);

        Ok(Some(Frame::Message {
            message: GopherRequest::decode(line),
            body: false,
        }))
    }

    fn encode(&mut self, frame: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        match frame {
            Frame::Message { message, .. } => {
                message.encode(buf)
            }
            Frame::Body { chunk } => {
                if let Some(chunk) = chunk {
                    buf.extend(chunk.as_slice());
                }
                Ok(())
            }
            Frame::Error { error } => {
                Err(error)
            }
        }
    }
}
