use nom::AsBytes;

pub fn prefix_with_size(v: Vec<u8>) -> Vec<u8> {
    let size = (v.len() as u32).to_be_bytes();
    [
        size.as_bytes(),
        &v,
    ].concat()
}

pub fn throttle_time_data() -> Vec<u8> {
    Vec::from(0_u32.to_be_bytes().as_bytes())
}

pub fn error_code_data() -> Vec<u8> {
    Vec::from(0_u16.to_be_bytes().as_bytes())
}

pub fn size_prefixed_string_data(s: &str) -> Vec<u8> {
    let size: &[u8] = &(s.len() as u16).to_be_bytes();
    [size, s.as_bytes()].concat()
}

pub fn port_data(p: u32) -> Vec<u8> {
    Vec::from(p.to_be_bytes().as_bytes())
}

pub fn null_string_data() -> Vec<u8> {
    Vec::from((-1_i16).to_be_bytes().as_bytes())
}

pub fn u32_data(num: u32) -> Vec<u8> {
    Vec::from(num.to_be_bytes().as_bytes())
}

pub fn bool_data(b: bool) -> Vec<u8> {
    let b: u8 = if b { 1 } else { 0 };
    Vec::from(b.to_be_bytes().as_bytes())
}