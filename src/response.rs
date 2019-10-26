use crate::api_versions::api_versions_response;
use crate::headers::RequestHeader;
use crate::join_group_request::JoinGroupRequest;
use crate::serialization_utils::{bool_data, error_code_data, null_string_data, port_data, prefix_with_size, size_prefixed_string_data, throttle_time_data, u32_data};
use std::ptr::null;

pub fn respond_to_request(request: RequestHeader, data: &[u8]) -> Vec<u8> {
    eprintln!("request = {:?}", request);
    match request.api_key {
        18 => api_versions_response(request.correlation_id),
        3 => metadata_response(request.correlation_id),
        10 => find_coordinator_response(request.correlation_id),
        11 => join_group(request, data),
        _ => Vec::new(),
    }
}

pub fn metadata_response(correlation_id: u32) -> Vec<u8> {
    let response: Vec<u8> = [
        u32_data(correlation_id),
        throttle_time_data(),
        u32_data(1),
        u32_data(0),
        size_prefixed_string_data("localhost"),
        port_data(9092),
        null_string_data(),
        null_string_data(),
        u32_data(0),
        u32_data(1),
        error_code_data(),
        size_prefixed_string_data("foo"),
        bool_data(false),
        u32_data(0),
        u32_data(0),
        u32_data(0),
    ].concat();

    prefix_with_size(response)
}

pub fn find_coordinator_response(correlation_id: u32) -> Vec<u8> {
    let response: Vec<u8> = [
        u32_data(correlation_id),
        throttle_time_data(),
        error_code_data(),
        null_string_data(),
        u32_data(0),
        size_prefixed_string_data("localhost"),
        port_data(9092),
    ].concat();

    prefix_with_size(response)
}

fn join_group(request: RequestHeader, data: &[u8]) -> Vec<u8> {
    let join_group_request = JoinGroupRequest::new(data);
    eprintln!("join_group_request = {:?}", join_group_request);
    Vec::new()
}