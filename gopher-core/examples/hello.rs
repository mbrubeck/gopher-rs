extern crate bytes;
extern crate futures;
extern crate gopher_core;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use futures::{future, Future, BoxFuture, Sink, Stream};
use gopher_core::codec::ServerCodec;
use gopher_core::{DirEntity, ItemType, GopherRequest, GopherResponse, GopherStr};
use std::io;
use tokio_io::AsyncRead;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use tokio_service::{NewService, Service};

pub struct HelloGopherServer;

impl Service for HelloGopherServer {
    type Request = GopherRequest;
    type Response = GopherResponse;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, request: Self::Request) -> Self::Future {
        println!("got request {:?}", request);

        let response = match &request.selector[..] {
            b"" => match request.query.as_ref() {
                None => GopherResponse::Menu(vec![
                    DirEntity {
                        item_type: ItemType::File,
                        name: GopherStr::from_latin1(b"Hello, world"),
                        selector: GopherStr::from_latin1(b"hello"),
                        host: GopherStr::from_latin1(b"0.0.0.0"),
                        port: 12345,
                    },
                    DirEntity {
                        item_type: ItemType::File,
                        name: GopherStr::from_latin1(b"Goodbye, world"),
                        selector: GopherStr::from_latin1(b"bye"),
                        host: GopherStr::from_latin1(b"0.0.0.0"),
                        port: 12345,
                    },
                ]),
                // Compatibility hack for gopher+ clients:
                Some(_) => GopherResponse::GopherPlusRedirect(DirEntity {
                    item_type: ItemType::Dir,
                    name: GopherStr::from_latin1(b"Main menu"),
                    selector: GopherStr::from_latin1(b""),
                    host: GopherStr::from_latin1(b"0.0.0.0"),
                    port: 12345,
                })
            },
            b"hello" => GopherResponse::TextFile(
                GopherStr::from_latin1(b"Hello, world.\r\nWelcome to Gopher.").into_buf()),
            b"bye" => GopherResponse::TextFile(GopherStr::from_latin1(b"Goodbye!").into_buf()),
            _ => GopherResponse::error(GopherStr::from_latin1(b"File not found")),
        };

        future::ok(response).boxed()
    }
}

fn serve<S>(s: S) -> io::Result<()>
    where S: NewService<Request = GopherRequest,
                        Response = GopherResponse,
                        Error = io::Error> + 'static
{
    let mut core = Core::new()?;
    let handle = core.handle();

    let address = "0.0.0.0:12345".parse().unwrap();
    let listener = TcpListener::bind(&address, &handle)?;

    let connections = listener.incoming();
    let server = connections.for_each(move |(socket, _peer_addr)| {
        let (writer, reader) = socket.framed(ServerCodec).split();
        let service = s.new_service()?;

        let response = reader.take(1).and_then(move |request| service.call(request));
        let server = writer.send_all(response).then(|_| Ok(()));
        handle.spawn(server);

        Ok(())
    });

    core.run(server)
}

fn main() {
    serve(|| Ok(HelloGopherServer)).unwrap();
}
