use alloc::vec;

use nonos_libc::mk_ipc_recv_from;

use crate::protocol::{
    parse, E_BAD_OP, E_INVAL, HDR_LEN, IPC_PAYLOAD_MAX, OP_DESCRIPTOR, OP_HEALTHCHECK, OP_PROBE,
};
use crate::input;
use crate::server::{handlers, respond};
use crate::state::State;

const SERVICE_INBOX: u64 = 0;
const RECV_TIMEOUT_MS: u64 = 2;

pub fn run(mut state: State) -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(
            SERVICE_INBOX,
            rx.as_mut_ptr(),
            rx.len(),
            RECV_TIMEOUT_MS,
            &mut sender_pid,
        );
        input::poll(&mut state);
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        let Some((req, body)) = parse(&rx[..n as usize]) else { continue };
        dispatch(&mut state, sender_pid, req, body, &mut tx);
    }
}

fn dispatch(
    state: &mut State,
    sender_pid: u32,
    req: crate::protocol::Request,
    body: &[u8],
    tx: &mut [u8],
) {
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(state, sender_pid, &req, tx),
        OP_PROBE if body.is_empty() => handlers::probe::handle(state, sender_pid, &req, tx),
        OP_DESCRIPTOR if body.is_empty() => {
            handlers::descriptor::handle(state, sender_pid, &req, tx)
        }
        _ if body.is_empty() => {
            let _ = respond::send(sender_pid, &req, E_BAD_OP, &[], tx);
        }
        _ => {
            let _ = respond::send(sender_pid, &req, E_INVAL, &[], tx);
        }
    }
}
