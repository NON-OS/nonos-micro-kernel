use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_ipc_call;

const MAGIC: u32 = 0x4E43_4D50;
const VERSION: u16 = 1;
const HDR_LEN: usize = 20;
const OP_HEALTHCHECK: u16 = 0x0001;
const OP_SCENE_SUBMIT: u16 = 0x0002;
const OP_DAMAGE_COMMIT: u16 = 0x0003;
const SCENE_REQ_LEN: usize = 32;
const DAMAGE_REQ_LEN: usize = 16;

fn call_status(port: u32, tx: &[u8]) -> Result<(), i32> {
    let mut rx = vec![0u8; HDR_LEN + 4];
    let rc = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if rc < (HDR_LEN + 4) as i64 {
        return Err(-11);
    }
    let status = i32::from_le_bytes(rx[HDR_LEN..HDR_LEN + 4].try_into().map_err(|_| -11)?);
    if status == 0 {
        Ok(())
    } else {
        Err(status)
    }
}

fn header(op: u16, request_id: u32, payload_len: u32) -> Vec<u8> {
    let mut tx = Vec::with_capacity(HDR_LEN + payload_len as usize);
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&VERSION.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&request_id.to_le_bytes());
    tx.extend_from_slice(&payload_len.to_le_bytes());
    tx
}

pub fn healthcheck(port: u32, request_id: u32) -> Result<(), i32> {
    call_status(port, &header(OP_HEALTHCHECK, request_id, 0))
}

pub fn push_scene_submit(
    port: u32,
    request_id: u32,
    surface_handle: u64,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    z: u32,
) -> Result<(), i32> {
    let mut tx = header(OP_SCENE_SUBMIT, request_id, SCENE_REQ_LEN as u32);
    tx.extend_from_slice(&surface_handle.to_le_bytes());
    tx.extend_from_slice(&x.to_le_bytes());
    tx.extend_from_slice(&y.to_le_bytes());
    tx.extend_from_slice(&width.to_le_bytes());
    tx.extend_from_slice(&height.to_le_bytes());
    tx.extend_from_slice(&z.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    call_status(port, &tx)
}

pub fn damage_commit(port: u32, request_id: u32, width: u32, height: u32) -> Result<(), i32> {
    let mut tx = header(OP_DAMAGE_COMMIT, request_id, DAMAGE_REQ_LEN as u32);
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&width.to_le_bytes());
    tx.extend_from_slice(&height.to_le_bytes());
    call_status(port, &tx)
}
