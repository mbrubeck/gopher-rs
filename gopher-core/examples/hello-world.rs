extern crate futures;
extern crate gopher_core;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use futures::{future, Future, BoxFuture};
use gopher_core::{DirEntity, ItemType};
use gopher_core::{GopherRequest, GopherResponse, GopherStr};
use gopher_core::proto::GopherServer;
use std::io;
use tokio_proto::TcpServer;
use tokio_service::Service;

pub struct HelloGopherServer;

impl Service for HelloGopherServer {
    type Request = GopherRequest;
    type Response = GopherResponse;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, request: Self::Request) -> Self::Future {
        println!("got request {:?}", request);

        let response = match &request.selector[..] {
            b"" => GopherResponse::Menu(vec![
                DirEntity {
                    item_type: ItemType::File,
                    name: GopherStr::from_latin1(b"hello, world"),
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
            b"hello" => GopherResponse::TextFile(
                GopherStr::from_latin1(b"Hello, world.\r\nWelcome to Gopher.").into_buf()),
            b"bye" => GopherResponse::TextFile(GopherStr::from_latin1(b"Goodbye").into_buf()),
            _ => GopherResponse::error(GopherStr::from_latin1(b"File not found")),
        };

        future::ok(response).boxed()
    }
}

fn main() {
    TcpServer::new(GopherServer, "0.0.0.0:12345".parse().unwrap())
        .serve(|| Ok(HelloGopherServer));
}
