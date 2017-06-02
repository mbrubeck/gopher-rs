use bytes::Bytes;
use codec::ServerCodec;
use futures::{stream, Poll, Stream, Sink, StartSend};
use std::io;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::streaming::pipeline::{ServerProto, Transport};
use types::{GopherRequest, GopherResponse, Void};

pub struct GopherServer;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for GopherServer {
    type Request = GopherRequest;
    type RequestBody = Void;

    type Response = GopherResponse;
    type ResponseBody = Bytes;

    type Error = io::Error;

    type Transport = OneShot<Framed<T, ServerCodec>>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(OneShot::new(io.framed(ServerCodec)))
    }
}

/// Transport that closes the stream after one request.
pub struct OneShot<S>(stream::Take<S>);

impl<S: Stream> OneShot<S> {
    fn new(stream: S) -> Self {
        OneShot(stream.take(1))
    }
}

impl<S: Stream> Stream for OneShot<S> {
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<S::Item>, S::Error> {
        self.0.poll()
    }
}

impl<S: Sink + Stream> Sink for OneShot<S> {
    type SinkItem = S::SinkItem;
    type SinkError = S::SinkError;

    fn start_send(&mut self, item: S::SinkItem) -> StartSend<S::SinkItem, S::SinkError> {
        self.0.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), S::SinkError> {
        self.0.poll_complete()
    }
}

impl<S> Transport for OneShot<S> where S: 'static +
                                          Stream<Error=io::Error> +
                                          Sink<SinkError = io::Error> {}
