use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::api_versions::api_versions_response;
use crate::parse_utils::read_u32;
use crate::request_header::RequestHeader;

mod parse_utils;
mod request_header;
mod api_versions;


fn process_stream(mut stream: TcpStream) {
    eprintln!("connected to = {:?}", stream.peer_addr().unwrap());
    let mut buffer = [0; 4096];
    loop {
        let nread = stream.read(&mut buffer);
        match nread {
            Ok(nread) => {
                if nread == 0 {
                    println!("disconnected!");
                    break;
                } else {
                    let data = &buffer[0..nread];
                    let size = read_u32(&data[0..4]) as usize;
                    let request_header = RequestHeader::new(&data[4..4 + size]);
                    eprintln!("request_header = {:?}", request_header);

                    let response = api_versions_response(request_header.correlation_id);
                    stream.write(response.as_slice()).expect("failed to respond!");
                }
            }
            Err(err) => {
                eprintln!("err = {:?}", err);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("localhost:9092")
        .expect("failed to initialize listener");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => process_stream(stream),
            Err(err) => eprintln!("err = {:?}", err),
        }
    }
}
