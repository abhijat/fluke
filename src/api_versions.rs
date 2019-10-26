use std::collections::HashMap;

fn serialized_api_versions() -> Vec<u8> {
    let mut api_versions = HashMap::new();

    api_versions.insert(18_u16, (0_u16, 2_u16));
    api_versions.insert(0_u16, (0_u16, 7_u16));
    api_versions.insert(3_u16, (0_u16, 8_u16));
    api_versions.insert(10_u16, (0_u16, 2_u16));
    api_versions.insert(11_u16, (0_u16, 5_u16));

    let mut data = Vec::<u8>::new();
    data.extend_from_slice(&(api_versions.len() as u32).to_be_bytes());

    for (id, (min_version, max_version)) in api_versions {
        data.extend_from_slice(&id.to_be_bytes());
        data.extend_from_slice(&min_version.to_be_bytes());
        data.extend_from_slice(&max_version.to_be_bytes());
    }

    data
}

pub fn api_versions_response(correlation_id: u32) -> Vec<u8> {
    let response: Vec<u8> = [
        &correlation_id.to_be_bytes() as &[u8],
        &0_u16.to_be_bytes(),
        serialized_api_versions().as_slice(),
        &0_u32.to_be_bytes(),
    ].concat();

    let size: &[u8] = &(response.len() as u32).to_be_bytes();
    [size, response.as_slice()].concat()
}
