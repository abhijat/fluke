use crate::api_versions::api_versions_response;
use crate::headers::RequestHeader;

pub fn respond_to_request(request: RequestHeader) -> Vec<u8> {
    eprintln!("request = {:?}", request);
    match request.api_key {
        18 => api_versions_response(request.correlation_id),
        _ => Vec::new(),
    }
}
