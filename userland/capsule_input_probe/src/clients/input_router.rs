use alloc::vec;

use nonos_libc::mk_ipc_call;

use crate::protocol::{encode_request, OP_GRAB_REQUEST, OP_SUBSCRIBE, REQ_HDR_LEN};

const KIND_MASK_KEYS: u32 = 0b11;
const BODY_LEN: usize = 8;
const STATUS_LEN: usize = 4;

fn call_status(router_port: u32, tx: &[u8]) -> Result<(), i32> {
    let mut rx = vec![0u8; REQ_HDR_LEN + STATUS_LEN];
    let rc = mk_ipc_call(router_port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if rc < (REQ_HDR_LEN + STATUS_LEN) as i64 {
        return Err(-11);
    }
    let status =
        i32::from_le_bytes(rx[REQ_HDR_LEN..REQ_HDR_LEN + STATUS_LEN].try_into().map_err(|_| -11)?);
    if status == 0 {
        Ok(())
    } else {
        Err(status)
    }
}

fn keys_request(out: &mut [u8], op: u16, request_id: u32) -> usize {
    let mut body = [0u8; BODY_LEN];
    body[0..4].copy_from_slice(&KIND_MASK_KEYS.to_le_bytes());
    encode_request(out, op, request_id, &body)
}

pub fn subscribe(router_port: u32, request_id: u32) -> Result<(), i32> {
    let mut buf = [0u8; REQ_HDR_LEN + BODY_LEN];
    let n = keys_request(&mut buf, OP_SUBSCRIBE, request_id);
    call_status(router_port, &buf[..n])
}

pub fn grab_keyboard(router_port: u32, request_id: u32) -> Result<(), i32> {
    let mut buf = [0u8; REQ_HDR_LEN + BODY_LEN];
    let n = keys_request(&mut buf, OP_GRAB_REQUEST, request_id);
    call_status(router_port, &buf[..n])
}
