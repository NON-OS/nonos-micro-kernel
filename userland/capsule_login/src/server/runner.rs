use alloc::vec;

use nonos_libc::mk_ipc_recv_from;

use crate::protocol::{
    parse, E_BAD_OP, E_INVAL, HDR_LEN, IPC_PAYLOAD_MAX, OP_END_SESSION, OP_GET_STATE,
    OP_HEALTHCHECK, OP_START_SESSION,
};
use crate::server::{handlers, respond};
use crate::state::Context;

const SERVICE_INBOX: u64 = 0;
const RECV_BLOCK: u64 = 0;
const RECV_NOWAIT: u64 = 1;

pub fn run(mut ctx: Context) -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        drain(&mut ctx, &mut rx, &mut tx);
    }
}

fn drain(ctx: &mut Context, rx: &mut [u8], tx: &mut [u8]) {
    let mut blocking = true;
    loop {
        let mut sender_pid = 0u32;
        let timeout = if blocking { RECV_BLOCK } else { RECV_NOWAIT };
        let n =
            mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), timeout, &mut sender_pid);
        if n <= 0 || sender_pid == 0 {
            return;
        }
        blocking = false;
        let (req, body) = match parse(&rx[..n as usize]) {
            Ok(v) => v,
            Err((req, errno)) => {
                let _ = respond::status(sender_pid, &req, errno, tx);
                continue;
            }
        };
        match req.op {
            OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender_pid, &req, tx),
            OP_START_SESSION => handlers::start_session::handle(ctx, sender_pid, &req, body, tx),
            OP_END_SESSION if body.is_empty() => {
                handlers::end_session::handle(ctx, sender_pid, &req, tx)
            }
            OP_GET_STATE if body.is_empty() => {
                handlers::get_state::handle(ctx, sender_pid, &req, tx)
            }
            _ if body.is_empty() => {
                let _ = respond::status(sender_pid, &req, E_BAD_OP, tx);
            }
            _ => {
                let _ = respond::status(sender_pid, &req, E_INVAL, tx);
            }
        }
    }
}
