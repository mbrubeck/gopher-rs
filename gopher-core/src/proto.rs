use codec;
use std::io;
use tokio_core::io::{Framed, Io};
use tokio_proto::pipeline::ServerProto;
use types::{GopherRequest, GopherResponse};

pub struct GopherServer;

impl<T: Io + 'static> ServerProto<T> for GopherServer {
    /// For this protocol style, `Request` matches the codec `In` type
    type Request = GopherRequest;

    /// For this protocol style, `Response` matches the coded `Out` type
    type Response = GopherResponse;

    /// A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, codec::Server>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(codec::Server))
    }
}
