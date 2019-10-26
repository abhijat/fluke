use crate::wire_parser::{parse_list, parse_protocol_metadata, parse_size, parse_string, parse_u32};

#[derive(Debug)]
pub struct JoinGroupRequest {
    group_id: String,
    session_timeout_ms: u32,
    rebalance_timeout_ms: u32,
    member_id: String,
    group_instance_id: Option<String>,
    protocol_type: String,
    protocols: Vec<(String, Vec<u8>)>,
}

impl JoinGroupRequest {
    pub fn new(data: &[u8]) -> Self {
        let (data, group_id) = parse_string(data);
        let (data, session_timeout_ms) = parse_u32(data);
        let (data, rebalance_timeout_ms) = parse_u32(data);
        let (data, member_id) = parse_string(data);
        let (data, group_instance_id) = parse_string(data);
        let (data, protocol_type) = parse_string(data);
        let (data, protocols) = parse_list(data, parse_protocol_metadata);

        JoinGroupRequest {
            group_id,
            session_timeout_ms,
            rebalance_timeout_ms,
            member_id,
            group_instance_id: Some(group_instance_id),
            protocol_type,
            protocols,
        }
    }
}