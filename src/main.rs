use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::api_versions::api_versions_response;
use crate::headers::RequestHeader;
use crate::parse_utils::read_u32;
use crate::response::respond_to_request;
use crate::wire_parser::{parse_array, parse_key, parse_request_header, parse_size, parse_size_prefixed_string};

mod api_versions;
mod parse_utils;
mod headers;
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

                    if header.api_key == 3 {
                        let (data, topics) = parse_array(data, parse_size_prefixed_string);
                        eprintln!("topics = {:?}", topics);
                    } else if header.api_key == 10 {
                        let (data, key) = parse_size_prefixed_string(data);
                        let (data, key_type) = parse_key(data);
                        eprintln!("FindCoordinator request for {} [{}]", key, key_type);
                    }

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
