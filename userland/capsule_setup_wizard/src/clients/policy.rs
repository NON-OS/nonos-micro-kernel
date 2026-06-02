use nonos_libc::mk_ipc_call;
use nonos_policy_proto::{Header, HDR_LEN, IPC_PAYLOAD_MAX, KIND_BOOL, KIND_I8, KIND_STR, KIND_U8, OP_SET};

fn finish(port: u32, tx: &[u8]) -> Result<(), i32> {
    let mut rx = [0u8; IPC_PAYLOAD_MAX];
    let n = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if n < HDR_LEN as i64 {
        return Err(-11);
    }
    let reply = Header::decode(&rx[..HDR_LEN]).ok_or(-11)?;
    if reply.status != 0 {
        return Err(reply.status as i32);
    }
    Ok(())
}

pub fn set_bool(port: u32, field: u32, value: bool) -> Result<(), i32> {
    let mut tx = [0u8; HDR_LEN + 1];
    let hdr = Header { op: OP_SET, field, kind: KIND_BOOL, status: 0, payload_len: 1 };
    hdr.encode(&mut tx[..HDR_LEN]);
    tx[HDR_LEN] = if value { 1 } else { 0 };
    finish(port, &tx)
}

pub fn set_u8(port: u32, field: u32, value: u8) -> Result<(), i32> {
    let mut tx = [0u8; HDR_LEN + 1];
    let hdr = Header { op: OP_SET, field, kind: KIND_U8, status: 0, payload_len: 1 };
    hdr.encode(&mut tx[..HDR_LEN]);
    tx[HDR_LEN] = value;
    finish(port, &tx)
}

pub fn set_i8(port: u32, field: u32, value: i8) -> Result<(), i32> {
    let mut tx = [0u8; HDR_LEN + 1];
    let hdr = Header { op: OP_SET, field, kind: KIND_I8, status: 0, payload_len: 1 };
    hdr.encode(&mut tx[..HDR_LEN]);
    tx[HDR_LEN] = value as u8;
    finish(port, &tx)
}

pub fn set_str(port: u32, field: u32, value: &[u8]) -> Result<(), i32> {
    let n = value.len().min(IPC_PAYLOAD_MAX - HDR_LEN);
    let mut tx = [0u8; IPC_PAYLOAD_MAX];
    let hdr = Header { op: OP_SET, field, kind: KIND_STR, status: 0, payload_len: n as u16 };
    hdr.encode(&mut tx[..HDR_LEN]);
    tx[HDR_LEN..HDR_LEN + n].copy_from_slice(&value[..n]);
    finish(port, &tx[..HDR_LEN + n])
}
