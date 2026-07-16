pub const MAGIC: u32 = 0x4E49_3243;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
pub const OP_TRANSFER: u16 = 5;
pub const OP_ACPI_HID: u16 = 7;
pub const OP_GPIO_DOORBELL: u16 = 8;
pub const FLAG_RESTART_ON_READ: u16 = 1 << 0;

pub fn request(seq: u64, addr: u8, write: &[u8], read_len: usize, out: &mut [u8]) -> usize {
    let body_len = 8 + write.len();
    let total = HDR_LEN + body_len;
    if out.len() < total {
        return 0;
    }
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&OP_TRANSFER.to_le_bytes());
    out[8..16].copy_from_slice(&seq.to_le_bytes());
    out[16..20].copy_from_slice(&(body_len as u32).to_le_bytes());
    out[20] = addr;
    out[21] = 0;
    out[22..24].copy_from_slice(&(write.len() as u16).to_le_bytes());
    out[24..26].copy_from_slice(&(read_len as u16).to_le_bytes());
    let flags = if !write.is_empty() && read_len != 0 { FLAG_RESTART_ON_READ } else { 0 };
    out[26..28].copy_from_slice(&flags.to_le_bytes());
    out[28..28 + write.len()].copy_from_slice(write);
    total
}

pub fn response(buf: &[u8], seq: u64, read: &mut [u8]) -> Option<usize> {
    if buf.len() < HDR_LEN + 12 || u32::from_le_bytes(buf[0..4].try_into().ok()?) != MAGIC {
        return None;
    }
    if u64::from_le_bytes(buf[8..16].try_into().ok()?) != seq {
        return None;
    }
    if i32::from_le_bytes(buf[20..24].try_into().ok()?) != 0 {
        return None;
    }
    let len = u16::from_le_bytes(buf[24..26].try_into().ok()?) as usize;
    if len > read.len() || 32 + len > buf.len() {
        return None;
    }
    read[..len].copy_from_slice(&buf[32..32 + len]);
    Some(len)
}
