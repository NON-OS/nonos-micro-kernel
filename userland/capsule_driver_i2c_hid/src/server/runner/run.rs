use alloc::vec;

use nonos_libc::mk_ipc_recv_from;

/* on-screen bring-up sentinels, silenced
const KIND_DRIVER_READY: u16 = 0xFE01;
const KIND_TOUCHPAD_FOUND: u16 = 0xFE02;
const KIND_CONTROLLER_BOUND: u16 = 0xFE04;
const KIND_BIND_CONFIRMED: u16 = 0xFE05;
const KIND_WAKE_CONFIRMED: u16 = 0xFE07;

fn signal(kind: u16) {
    let ev =
        InputEvent { kind, flags: 0, code: 0, x: 0, y: 0, delta_x: 0, delta_y: 0, timestamp_ns: 0 };
    let _ = mk_input_event_post(&ev);
}
*/

use super::dispatch::dispatch;
use crate::input;
use crate::protocol::{parse, HDR_LEN, IPC_PAYLOAD_MAX};
use crate::state::State;

const SERVICE_INBOX: u64 = 0;
const RECV_TIMEOUT_MS: u64 = 2;
// A touchpad may not answer the very first probe at startup: the I2C
// controller has just come up, and the device needs a moment to wake. So
// keep re-probing on a slow cadence until it is found, instead of giving up
// after the single startup attempt and never producing input.
const REPROBE_EVERY: u32 = 250;

pub fn run(mut state: State) -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut ticks: u32 = 0;
    /* bring-up sentinels, silenced
    let mut found_signaled = false;
    signal(KIND_DRIVER_READY);
    if let Some((slot, confirmed)) = crate::i2c_client::query_driver_info(state.i2c_port) {
        for _ in 0..slot {
            signal(KIND_CONTROLLER_BOUND);
        }
        if confirmed {
            signal(KIND_BIND_CONFIRMED);
        }
    }
    */
    loop {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(
            SERVICE_INBOX,
            rx.as_mut_ptr(),
            rx.len(),
            RECV_TIMEOUT_MS,
            &mut sender_pid,
        );
        if !state.found() {
            ticks = ticks.wrapping_add(1);
            if ticks.is_multiple_of(REPROBE_EVERY) {
                crate::setup::reprobe(&mut state);
            }
        }
        /* bring-up sentinels, silenced
        if state.found() && !found_signaled {
            signal(KIND_TOUCHPAD_FOUND);
            if state.woke {
                signal(KIND_WAKE_CONFIRMED);
            }
            found_signaled = true;
        }
        */
        input::poll(&mut state);
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        let Some((req, body)) = parse(&rx[..n as usize]) else { continue };
        dispatch(&mut state, sender_pid, req, body, &mut tx);
    }
}
