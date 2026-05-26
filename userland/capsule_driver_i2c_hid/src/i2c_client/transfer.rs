use alloc::vec;

use nonos_libc::mk_ipc_call_timeout;

use crate::i2c_client::{seq, wire};

pub fn write_read(port: u32, addr: u8, write: &[u8], read: &mut [u8]) -> Option<usize> {
    let request_id = seq::next();
    let mut tx = vec![0u8; 96];
    let n = wire::request(request_id, addr, write, read.len(), &mut tx);
    if n == 0 {
        return None;
    }
    let mut rx = vec![0u8; 128];
    let got = mk_ipc_call_timeout(port as u64, tx.as_ptr(), n, rx.as_mut_ptr(), rx.len(), 250);
    if got <= 0 {
        return None;
    }
    wire::response(&rx[..got as usize], request_id, read)
}
