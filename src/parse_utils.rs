use std::convert::TryInto;

pub fn read_u16(data: &[u8]) -> u16 {
    u16::from_be_bytes(data[0..2].try_into().unwrap())
}

pub fn read_u32(data: &[u8]) -> u32 {
    u32::from_be_bytes(data[0..4].try_into().unwrap())
}
