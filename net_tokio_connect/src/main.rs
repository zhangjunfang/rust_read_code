extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;
extern crate bytes;


use tokio_codec::{Decoder, BytesCodec};
use tokio::net::TcpListener;
use tokio::prelude::Stream;
use tokio::prelude::Future;
use bytes::BytesMut;
//use bytes::Bytes;
use std::env;
use std::net::SocketAddr;


fn main() {
//    let a = Bytes::from(&b"hello world"[..]);
//    let b = a.slice(0, a.len());
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();
    let socket = TcpListener::bind(&addr).unwrap();
    let done = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            let framed = BytesCodec::new().framed(socket);
            let (_writer, reader) = framed.split();

            let processor = reader.for_each(|bytes| {
                let m:BytesMut=bytes;
                let arry=m.split(|arry| *arry == 124);//|分割的字符串
                for c in arry {
                    let cc=String::from_utf8_lossy(c);
                    let tt=String::from(cc);
                    println!("String内容：：：{:?} ",tt);
                    writer.
                }
                Ok(())
                })
                .and_then(|()| {
                    println!("Socket received FIN packet and closed connection");
                    Ok(())
                })
                .or_else(|err| {
                    println!("Socket closed with error: {:?}", err);
                    Err(err)
                })
                .then(|result| {
                    println!("Socket closed with result: {:?}", result);
                    Ok(())
                });
            tokio::spawn(processor)
        });
    tokio::run(done);
}