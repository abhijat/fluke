use crate::parse_utils::{read_u16, read_u32};

#[derive(Debug)]
pub struct RequestHeader {
    pub api_key: u16,
    pub api_version: u16,
    pub correlation_id: u32,
    pub client_id: String,
}

impl RequestHeader {
    pub fn new(data: &[u8]) -> Self {
        let api_key = read_u16(&data[0..2]);
        let api_version = read_u16(&data[2..4]);
        let correlation_id = read_u32(&data[4..8]);
        let client_id_size = read_u16(&data[8..10]) as usize;
        let client_id = String::from_utf8_lossy(&data[10..10 + client_id_size]);

        RequestHeader {
            api_key,
            api_version,
            correlation_id,
            client_id: String::from(client_id),
        }
    }
}
