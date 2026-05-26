use alloc::vec::Vec;

use nonos_libc::mk_ipc_call_timeout;

use super::{build_request, reply::decode_status, BOOT_REPLY_TIMEOUT_MS, CALL_REPLY_TIMEOUT_MS, NCMP_HDR_LEN};

pub(crate) fn call(compositor_port: u32, op: u16, request_id: u32, payload: &[u8]) -> Result<i32, &'static str> {
    call_with_timeout(compositor_port, op, request_id, payload, CALL_REPLY_TIMEOUT_MS)
}

pub(crate) fn call_boot(
    compositor_port: u32,
    op: u16,
    request_id: u32,
    payload: &[u8],
) -> Result<i32, &'static str> {
    call_with_timeout(compositor_port, op, request_id, payload, BOOT_REPLY_TIMEOUT_MS)
}

fn call_with_timeout(
    compositor_port: u32,
    op: u16,
    request_id: u32,
    payload: &[u8],
    timeout_ms: u64,
) -> Result<i32, &'static str> {
    let mut tx = Vec::with_capacity(NCMP_HDR_LEN + payload.len());
    build_request(&mut tx, op, request_id, payload);
    let mut rx = [0u8; NCMP_HDR_LEN + 4];
    let rc = mk_ipc_call_timeout(
        compositor_port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        timeout_ms,
    );
    decode_status(&rx, op, request_id, rc)
}
