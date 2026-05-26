use alloc::vec::Vec;

use nonos_libc::mk_ipc_call_timeout;

use super::{build_request, reply::decode_status_payload, BOOT_REPLY_TIMEOUT_MS, NCMP_HDR_LEN};

pub(crate) fn call_payload_boot(
    compositor_port: u32,
    op: u16,
    request_id: u32,
    payload: &[u8],
    body: &mut [u8],
) -> Result<i32, &'static str> {
    let mut tx = Vec::with_capacity(NCMP_HDR_LEN + payload.len());
    build_request(&mut tx, op, request_id, payload);
    let mut rx = Vec::with_capacity(NCMP_HDR_LEN + 4 + body.len());
    rx.resize(NCMP_HDR_LEN + 4 + body.len(), 0);
    let rc = mk_ipc_call_timeout(
        compositor_port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        BOOT_REPLY_TIMEOUT_MS,
    );
    decode_status_payload(&rx, op, request_id, body, rc)
}
