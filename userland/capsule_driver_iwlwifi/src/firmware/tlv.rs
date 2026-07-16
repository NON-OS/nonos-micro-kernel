use crate::constants::{
    FW_API_VERSION_MASK, IWL_FW_MAGIC, MAX_FW_API_VERSION, MIN_FW_API_VERSION,
};

// TLV type numbers from enum iwl_ucode_tlv_type (Linux iwlwifi fw/file.h),
// pinned to the real embedded firmware by blob_scan_tests: the 7265 image
// carries its runtime sections under type 19.
pub const TLV_SEC_RT: u32 = 19;
pub const TLV_SEC_INIT: u32 = 20;
pub const TLV_PAGING: u32 = 32;

/// The TLV ucode header is 88 bytes: a zero word, the magic, a 64-byte
/// human-readable name, then the version and build. The TLV records begin
/// immediately after it.
pub const TLV_HEADER_LEN: usize = 88;

#[derive(Clone, Copy)]
pub struct Header {
    pub major: u16,
    pub minor: u16,
    pub api: u16,
    pub build: u32,
}

pub fn parse_header(data: &[u8]) -> Option<Header> {
    if data.len() < TLV_HEADER_LEN {
        return None;
    }
    let zero = le32(data, 0)?;
    let magic = le32(data, 4)?;
    if zero != 0 || magic != IWL_FW_MAGIC {
        return None;
    }
    // The version and build sit after the 64-byte human-readable name (offset
    // 8..72), not right after the magic. Reading them at offset 8 would parse
    // the name string, which is exactly the bug the real-blob proof guards.
    let ver = le32(data, 72)?;
    let api = (ver & FW_API_VERSION_MASK) as u16;
    if !(MIN_FW_API_VERSION..=MAX_FW_API_VERSION).contains(&api) {
        return None;
    }
    Some(Header {
        major: ((ver >> 24) & 0xFF) as u16,
        minor: ((ver >> 16) & 0xFF) as u16,
        api,
        build: le32(data, 76)?,
    })
}

pub fn le32(data: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(data.get(off..off + 4)?.try_into().ok()?))
}
