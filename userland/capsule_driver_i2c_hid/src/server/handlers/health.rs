use crate::protocol::{Request, E_OK};
use crate::server::respond;
use crate::state::State;

pub fn handle(state: &State, sender_pid: u32, req: &Request, out: &mut [u8]) {
    let mut body = [0u8; 56];
    body[0] = state.found() as u8;
    body[1] = state.addr;
    body[4..8].copy_from_slice(&state.i2c_port.to_le_bytes());
    body[8..12].copy_from_slice(&state.i2c_pid.to_le_bytes());
    body[16..24].copy_from_slice(&state.probes.to_le_bytes());
    body[24..26].copy_from_slice(&state.input_register.to_le_bytes());
    body[26..28].copy_from_slice(&(state.input_len as u16).to_le_bytes());
    body[32..40].copy_from_slice(&state.input_polls.to_le_bytes());
    body[40..48].copy_from_slice(&state.input_reports.to_le_bytes());
    body[48..56].copy_from_slice(&state.post_failures.to_le_bytes());
    let _ = respond::send(sender_pid, req, E_OK, &body, out);
}
