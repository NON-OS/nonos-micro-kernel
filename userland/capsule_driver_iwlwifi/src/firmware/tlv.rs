use crate::constants::{
    FW_API_VERSION_MASK, IWL_FW_MAGIC, MAX_FW_API_VERSION, MIN_FW_API_VERSION,
};

pub const TLV_SEC_RT: u32 = 20;
pub const TLV_SEC_INIT: u32 = 21;
pub const TLV_PAGING: u32 = 33;

#[derive(Clone, Copy)]
pub struct Header {
    pub major: u16,
    pub minor: u16,
    pub api: u16,
    pub build: u32,
}

pub fn parse_header(data: &[u8]) -> Option<Header> {
    if data.len() < 20 {
        return None;
    }
    let zero = le32(data, 0)?;
    let magic = le32(data, 4)?;
    if zero != 0 || magic != IWL_FW_MAGIC {
        return None;
    }
    let ver = le32(data, 8)?;
    let api = (ver & FW_API_VERSION_MASK) as u16;
    if !(MIN_FW_API_VERSION..=MAX_FW_API_VERSION).contains(&api) {
        return None;
    }
    Some(Header {
        major: ((ver >> 24) & 0xFF) as u16,
        minor: ((ver >> 16) & 0xFF) as u16,
        api,
        build: le32(data, 12)?,
    })
}

pub fn le32(data: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(data.get(off..off + 4)?.try_into().ok()?))
}
