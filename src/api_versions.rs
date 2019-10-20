use std::collections::HashMap;

fn serialized_api_versions() -> Vec<u8> {
    let mut apis = HashMap::new();
    apis.insert(18_u16, (0_u16, 2_u16));

    let mut data = Vec::<u8>::new();
    data.extend_from_slice(&(apis.len()).to_be_bytes());

    for (id, (min_version, max_version)) in apis {
        data.extend_from_slice(&id.to_be_bytes());
        data.extend_from_slice(&min_version.to_be_bytes());
        data.extend_from_slice(&max_version.to_be_bytes());
    }

    data
}

pub fn api_versions_response(correlation_id: u32) -> Vec<u8> {
    let mut response_payload = Vec::<u8>::new();

    // the correlation_id
    response_payload.extend_from_slice(&correlation_id.to_be_bytes());

    // The error
    response_payload.extend_from_slice(&0_u16.to_be_bytes());

    response_payload.extend(serialized_api_versions());

    // throttle
    response_payload.extend_from_slice(&0_u32.to_be_bytes());

    // prepend the size of the payload to itself
    let size: &[u8] = &(response_payload.len() as u32).to_be_bytes();
    let mut response = Vec::<u8>::from(size);

    eprintln!("response = {:?}", response);
    eprintln!("response_payload = {:?}", response_payload);
    eprintln!("response_payload.len() = {:?}", response_payload.len());

    response.extend(response_payload);
    response
}