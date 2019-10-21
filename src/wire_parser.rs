use nom::bytes::streaming::take;
use nom::IResult;
use nom::number::complete::{be_u16, be_u32, be_u8};
use nom::sequence::tuple;

use crate::headers::RequestHeader;

pub fn parse_key(data: &[u8]) -> (&[u8], u8) {
    let result: IResult<&[u8], u8> = be_u8(data);
    result.expect("Failed to parse size from data")
}

pub fn parse_size(data: &[u8]) -> (&[u8], u32) {
    let result: IResult<&[u8], u32> = be_u32(data);
    result.expect("Failed to parse size from data")
}

pub fn parse_list<T>(data: &[u8], entity_parser: fn(&[u8]) -> (&[u8], T)) -> (&[u8], Vec<T>) {
    // first read the size
    let (data, size_of_array) = parse_size(data);

    let mut results = Vec::<T>::new();
    for i in 0..size_of_array {
        let (data, item) = entity_parser(data);
        results.push(item);
    }

    (data, results)
}

pub fn parse_string(data: &[u8]) -> (&[u8], String) {
    let result: IResult<&[u8], u16> = be_u16(data);
    let (data, string_size) = result.expect("Failed to extract string size");

    let result: IResult<&[u8], &[u8]> = take(string_size)(data);
    let (data, string) = result.expect(&format!("Failed to extract string of size {}", string_size));

    (data, String::from_utf8_lossy(string).to_string())
}

pub fn parse_request_header(data: &[u8]) -> (&[u8], RequestHeader) {
    // The header format is: api-key | api-version | correlation-id | size-of-client-id | client-id
    let header_parser = tuple((be_u16, be_u16, be_u32));

    let result: IResult<&[u8], (u16, u16, u32)> = header_parser(data);
    let (data, header) = result.expect("Failed to parse request header");

    let (api_key, api_version, correlation_id) = header;

    // We need to parse the client-id separately, once we have its size
    let (data, client_id) = parse_string(data);

    let request_header = RequestHeader::new(api_key, api_version, correlation_id, client_id);
    (data, request_header)
}

pub fn parse_boolean(data: &[u8]) -> (&[u8], bool) {
    let result: IResult<&[u8], u8> = be_u8(data);
    let (data, value) = result.expect("Failed to parse boolean");
    (data, value > 0)
}