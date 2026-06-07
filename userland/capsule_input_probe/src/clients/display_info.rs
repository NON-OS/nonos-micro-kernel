use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_ipc_call;

const MAGIC: u32 = 0x4E43_4D50;
const VERSION: u16 = 1;
const HDR_LEN: usize = 20;
const OP_DISPLAY_INFO: u16 = 0x0008;
const RESP_LEN: usize = HDR_LEN + 4 + 16;

pub struct DisplayInfo {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

fn header(op: u16, request_id: u32) -> Vec<u8> {
    let mut tx = Vec::with_capacity(HDR_LEN);
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&VERSION.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&request_id.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx
}

fn read_u32(rx: &[u8], off: usize) -> Result<u32, i32> {
    Ok(u32::from_le_bytes(rx[off..off + 4].try_into().map_err(|_| -11)?))
}

pub fn query_display_info(port: u32, request_id: u32) -> Result<DisplayInfo, i32> {
    let tx = header(OP_DISPLAY_INFO, request_id);
    let mut rx = vec![0u8; RESP_LEN];
    let rc = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if rc < RESP_LEN as i64 {
        return Err(-11);
    }
    let status = read_u32(&rx, HDR_LEN)? as i32;
    if status != 0 {
        return Err(status);
    }
    let width = read_u32(&rx, HDR_LEN + 4)?;
    let height = read_u32(&rx, HDR_LEN + 8)?;
    let stride = read_u32(&rx, HDR_LEN + 12)?;
    // Reject a stride that cannot hold a full ARGB8888 row, otherwise the fill
    // loop would write past the mapped surface.
    if width == 0 || height == 0 || stride == 0 || (stride as u64) < (width as u64) * 4 {
        return Err(-22);
    }
    Ok(DisplayInfo { width, height, stride })
}
