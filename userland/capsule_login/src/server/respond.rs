use nonos_libc::mk_ipc_reply;

use crate::protocol::{response_header, write_status, Request, HDR_LEN, STATUS_LEN};

pub fn status(sender_pid: u32, req: &Request, errno: i32, tx: &mut [u8]) -> i64 {
    response_header(tx, req, STATUS_LEN as u32);
    write_status(tx, errno);
    mk_ipc_reply(sender_pid, tx.as_ptr(), HDR_LEN + STATUS_LEN)
}

pub fn payload(sender_pid: u32, req: &Request, body_len: usize, tx: &mut [u8]) -> i64 {
    response_header(tx, req, (STATUS_LEN + body_len) as u32);
    write_status(tx, 0);
    mk_ipc_reply(sender_pid, tx.as_ptr(), HDR_LEN + STATUS_LEN + body_len)
}
