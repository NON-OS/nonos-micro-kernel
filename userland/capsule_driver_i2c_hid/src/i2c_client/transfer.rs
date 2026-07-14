use alloc::vec;

use nonos_libc::mk_ipc_call_timeout;

use crate::i2c_client::{seq, wire};

pub fn write_read(port: u32, addr: u8, write: &[u8], read: &mut [u8]) -> Option<usize> {
    let request_id = seq::next();
    // Request frame: header + the 8-byte transfer body header + the write bytes.
    let mut tx = vec![0u8; wire::HDR_LEN + 8 + write.len()];
    let n = wire::request(request_id, addr, write, read.len(), &mut tx);
    if n == 0 {
        return None;
    }
    // Response frame: the read data lands at a fixed 32-byte offset, so size the
    // reply buffer to the requested read length plus that offset. A fixed cap
    // here silently truncates large reads (the HID report descriptor above all).
    let mut rx = vec![0u8; 32 + read.len()];
    let got = mk_ipc_call_timeout(port as u64, tx.as_ptr(), n, rx.as_mut_ptr(), rx.len(), 250);
    if got <= 0 {
        return None;
    }
    wire::response(&rx[..got as usize], request_id, read)
}
