use crate::protocol::{E_BAD_OP, E_INVAL, OP_DESCRIPTOR, OP_HEALTHCHECK, OP_PROBE};
use crate::server::{handlers, respond};
use crate::state::State;

pub(super) fn dispatch(
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
