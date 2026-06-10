// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Boot-time runtime proof for the PS/2 input driver capsule.
//! Drives healthcheck, get_state on a fresh ring, then polls for
//! events from a QEMU `sendkey` injection. The smoke harness
//! injects scancodes via the QEMU monitor; the capsule's IRQ
//! handler picks them up, drains the controller, and the kernel
//! client surfaces them through `poll_events`. Emits the marker
//! set `tests/boot/ps2_input_round_trip.sh` greps for. Gated on
//! `nonos-driver-ps2-input-smoketest`.

use crate::services::lifecycle::smoketest_log;

use super::client;
use super::error::DriverPs2Error;
use super::state;

const TAG: &[u8] = b"[DRIVER-PS2-TEST] ";
const POLL_WINDOW_MS: u64 = 30_000;
const POLL_YIELD_BETWEEN: u32 = 32;

pub fn run() {
    if !state::is_alive() {
        return fail_msg(b"capsule not alive");
    }
    mark(b"capsule alive");

    if let Err(e) = client::healthcheck() {
        return fail(b"healthcheck", e);
    }
    mark(b"healthcheck ok");

    let initial = match client::get_state() {
        Ok(s) => s,
        Err(e) => return fail(b"get_state initial", e),
    };
    if initial.events_dropped != 0 || initial.parity_errors != 0 || initial.timeout_errors != 0 {
        return fail_msg(b"fresh ring reported nonzero error counters");
    }
    mark(b"get_state ok");

    let mut total_events: u32 = 0;
    let deadline = crate::time::timestamp_millis().saturating_add(POLL_WINDOW_MS);
    while crate::time::timestamp_millis() < deadline {
        match client::poll_events() {
            Ok(events) => {
                total_events = total_events.saturating_add(events.len() as u32);
                if total_events > 0 {
                    break;
                }
            }
            Err(e) => return fail(b"poll_events", e),
        }
        for _ in 0..POLL_YIELD_BETWEEN {
            crate::sched::yield_now();
        }
    }
    if total_events == 0 {
        return fail_msg(b"no scancodes after sendkey injection");
    }
    mark(b"poll_events ok");

    let after = match client::get_state() {
        Ok(s) => s,
        Err(e) => return fail(b"get_state after", e),
    };
    if after.events_seen <= initial.events_seen {
        return fail_msg(b"events_seen did not advance");
    }
    mark(b"counters advanced");

    mark(b"PASS");
}

fn mark(stage: &[u8]) {
    smoketest_log::mark(TAG, stage);
}

fn fail(stage: &[u8], err: DriverPs2Error) {
    smoketest_log::fail_with_err(TAG, stage, err_name(err));
}

fn fail_msg(reason: &[u8]) {
    smoketest_log::fail_msg(TAG, reason);
}

fn err_name(e: DriverPs2Error) -> &'static [u8] {
    match e {
        DriverPs2Error::Dead => b"Dead",
        DriverPs2Error::Stale => b"Stale",
        DriverPs2Error::AccessDenied => b"AccessDenied",
        DriverPs2Error::InvalidArgument => b"InvalidArgument",
        DriverPs2Error::DeviceFailure => b"DeviceFailure",
        DriverPs2Error::NoCallerPid => b"NoCallerPid",
        DriverPs2Error::TransportFailure => b"TransportFailure",
        DriverPs2Error::ProtocolMismatch => b"ProtocolMismatch",
    }
}
