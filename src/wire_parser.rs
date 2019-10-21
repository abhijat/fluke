use nom::bytes::streaming::take;
use nom::number::complete::{be_u16, be_u32};
use nom::sequence::tuple;
use nom::IResult;

use crate::request_header::RequestHeader;

pub fn parse_size(data: &[u8]) -> (&[u8], u32) {
    let result: IResult<&[u8], u32> = be_u32(data);
    result.expect("Failed to parse size from data")
}

pub fn parse_request_header(data: &[u8]) -> (&[u8], RequestHeader) {
    let header_parser = tuple((be_u16, be_u16, be_u32, be_u16));

    let result: IResult<&[u8], (u16, u16, u32, u16)> = header_parser(data);
    let (data, header) = result.expect("Failed to parse request header");
    let (api_key, api_version, correlation_id, client_id_size) = header;

    let result: IResult<&[u8], &[u8]> = take(client_id_size)(data);
    let (data, client_id) = result.expect("Failed to parse client_id");

    let client_id = String::from_utf8_lossy(client_id).to_string();

    let request_header = RequestHeader::new(api_key, api_version, correlation_id, client_id);
    (data, request_header)
}
