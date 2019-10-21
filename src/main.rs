use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::api_versions::api_versions_response;
use crate::parse_utils::read_u32;
use crate::request_header::RequestHeader;
use crate::response::respond_to_request;
use crate::wire_parser::{parse_request_header, parse_size};

mod api_versions;
mod parse_utils;
mod request_header;
mod response;
mod wire_parser;

fn process_stream(mut stream: TcpStream) {
    let peer = stream.peer_addr().expect("failed to get peer address");

    eprintln!("connected to {}", peer);
    let mut buffer = [0; 4096];

    loop {
        let nread = stream.read(&mut buffer);
        match nread {
            Ok(nread) => {
                if nread == 0 {
                    eprintln!("disconnected from {}", peer);
                    break;
                } else {
                    let data = &buffer[0..nread];
                    let (data, size) = parse_size(data);
                    let (data, header) = parse_request_header(data);
                    let response = respond_to_request(header);
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
    let listener = TcpListener::bind("localhost:9092").expect("failed to initialize listener");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => process_stream(stream),
            Err(err) => eprintln!("err = {:?}", err),
        }
    }
}
