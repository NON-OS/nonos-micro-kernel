use nonos_libc::mk_ipc_call;
use nonos_policy_proto::{Header, HDR_LEN, IPC_PAYLOAD_MAX, KIND_BOOL, OP_SET};

pub fn set_bool(port: u32, field: u32, value: bool) -> Result<(), i32> {
    let mut tx = [0u8; HDR_LEN + 1];
    let hdr = Header { op: OP_SET, field, kind: KIND_BOOL, status: 0, payload_len: 1 };
    hdr.encode(&mut tx[..HDR_LEN]);
    tx[HDR_LEN] = if value { 1 } else { 0 };
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
