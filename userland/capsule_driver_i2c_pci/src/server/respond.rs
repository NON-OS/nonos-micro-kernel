use crate::protocol::{response, Request};
use nonos_libc::mk_ipc_reply;

pub fn send(sender_pid: u32, req: &Request, errno: i32, body: &[u8], out: &mut [u8]) -> i64 {
    let n = response(req.op, req.request_id, errno, body, out);
    mk_ipc_reply(sender_pid, out.as_ptr(), n)
}
