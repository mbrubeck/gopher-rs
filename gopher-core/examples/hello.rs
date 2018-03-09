extern crate futures;
extern crate gopher_core;
extern crate tokio;
extern crate tokio_io;

use futures::{future, Future, Sink, Stream};
use gopher_core::codec::ServerCodec;
use gopher_core::{DirEntity, ItemType, GopherRequest, GopherResponse, GopherStr};
use std::io;
use tokio::{net::TcpListener, io::AsyncRead};

fn process(request: GopherRequest) -> future::FutureResult<GopherResponse, io::Error> {
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

    future::ok(response)
}

fn serve() -> io::Result<()> {
    let address = "0.0.0.0:12345".parse().unwrap();
    let listener = TcpListener::bind(&address)?;

    let server = listener.incoming()
        .map_err(|e| eprintln!("{:?}", e))
        .for_each(move |socket| {
            let (writer, reader) = socket.framed(ServerCodec).split();

            let response = reader.take(1).and_then(move |request| process(request));
            let server = writer.send_all(response).then(|_| Ok(()));
            tokio::spawn(server);

            Ok(())
        });

    tokio::run(server);
    Ok(())
}

fn main() { serve().unwrap(); }
