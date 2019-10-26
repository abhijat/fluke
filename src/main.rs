use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

use crate::response::respond_to_request;
use crate::wire_parser::{parse_boolean, parse_key, parse_list, parse_request_header, parse_size, parse_string};

mod api_versions;
mod join_group_request;
mod headers;
mod response;
mod wire_parser;
mod serialization_utils;

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
                        let (data, topics) = parse_list(data, parse_string);
                        let (data, allow_auto_topic_creation) = parse_boolean(data);
                        let (data, include_cluster_authorized_operations) = parse_boolean(data);
                        let (data, include_topic_authorized_operations) = parse_boolean(data);
                        eprintln!("topics = {:?} | flags = {:?}", topics, (allow_auto_topic_creation, include_topic_authorized_operations, include_topic_authorized_operations));
                    } else if header.api_key == 10 {
                        let (data, key) = parse_string(data);
                        let (data, key_type) = parse_key(data);
                        eprintln!("FindCoordinator request for {} [{}]", key, key_type);
                    }

                    let response = respond_to_request(header, data);
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
            Ok(stream) => {
                spawn(move || {
                    process_stream(stream);
                });
            }
            Err(err) => eprintln!("err = {:?}", err),
        }
    }
}
