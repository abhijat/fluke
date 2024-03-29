#[derive(Debug)]
pub struct RequestHeader {
    pub api_key: u16,
    pub api_version: u16,
    pub correlation_id: u32,
    pub client_id: String,
}

impl RequestHeader {
    pub fn new(api_key: u16, api_version: u16, correlation_id: u32, client_id: String) -> Self {
        RequestHeader {
            api_key,
            api_version,
            correlation_id,
            client_id,
        }
    }
}

#[derive(Debug)]
pub struct ResponseHeader {
    pub correlation_id: u32,
}

impl ResponseHeader {
    pub fn new(correlation_id: u32) -> Self {
        ResponseHeader { correlation_id }
    }
}