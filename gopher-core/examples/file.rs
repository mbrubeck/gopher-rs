extern crate bytes;
extern crate futures;
extern crate gopher_core;
extern crate tokio_proto;
extern crate tokio_service;

use futures::{future, Future, BoxFuture, Sink};
use gopher_core::{DirEntity, ItemType};
use gopher_core::{GopherRequest, GopherResponse, GopherStr, Void};
use gopher_core::proto::GopherServer;
use std::io;
use std::io::prelude::*;
use std::thread;
use std::fs::File;
use bytes::Bytes;
use tokio_proto::TcpServer;
use tokio_proto::streaming::{Body, Message};
use tokio_service::Service;

pub struct HelloGopherServer;

impl Service for HelloGopherServer {
    type Request = Message<GopherRequest, Body<Void, io::Error>>;
    type Response = Message<GopherResponse, Body<Bytes, io::Error>>;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, message: Self::Request) -> Self::Future {
        let request = match message {
            Message::WithoutBody(request) => request,
            _ => unreachable!(),
        };

        println!("got request {:?}", request);

        let response = match &request.selector[..] {
            b"" => match request.query.as_ref() {
                None => GopherResponse::Menu(vec![
                    DirEntity {
                        item_type: ItemType::File,
                        name: GopherStr::from_latin1(b"Download file"),
                        selector: GopherStr::from_latin1(b"file"),
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
            b"file" => {
                let (mut tx, body) = Body::pair();
                thread::spawn(move || {
                    let filename = std::env::args_os().next().unwrap();
                    let mut f = File::open(filename).unwrap();
                    let mut buf = [0; 1024];
                    while let Ok(n) = f.read(&mut buf) {
                        if n == 0 { break }
                        tx = tx.send(Ok(Bytes::from(&buf[..n]))).wait().unwrap();
                    }
                });
                let response = GopherResponse::BinaryFile(Bytes::new());
                return future::ok(Message::WithBody(response, body)).boxed()
            }
            _ => GopherResponse::error(GopherStr::from_latin1(b"File not found")),
        };

        future::ok(Message::WithoutBody(response)).boxed()
    }
}

fn main() {
    println!("listening on port 12345");
    TcpServer::new(GopherServer, "0.0.0.0:12345".parse().unwrap())
        .serve(|| Ok(HelloGopherServer));
}
