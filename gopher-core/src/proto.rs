use codec;
use futures::{stream, Stream};
use std::io;
use tokio_core::io::{Io, Framed};
use tokio_core::net::TcpStream;
use tokio_proto::pipeline::ServerProto;
use types::{GopherRequest, GopherResponse};

pub struct GopherServer;

impl ServerProto<TcpStream> for GopherServer {
    /// For this protocol style, `Request` matches the codec `In` type
    type Request = GopherRequest;

    /// For this protocol style, `Response` matches the coded `Out` type
    type Response = GopherResponse;

    /// A bit of boilerplate to hook in the codec:
    type Transport = stream::Take<Framed<TcpStream, codec::Server>>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: TcpStream) -> Self::BindTransport {
        // Use .take() to close the stream after a single response.
        Ok(io.framed(codec::Server).take(1))
    }
}
