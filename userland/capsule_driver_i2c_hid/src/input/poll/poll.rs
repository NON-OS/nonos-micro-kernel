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

use super::signal_raw_report::signal_raw_report;
use crate::hid::decode_touch;
use crate::i2c_client::write_read;
use crate::input::parse_report::parse_report;
use crate::input::publish::publish;
use crate::input::publish_touch::publish_touch;
use crate::state::State;

pub fn poll(state: &mut State) {
    if !state.found() || state.input_register == 0 {
        return;
    }
    // A full Precision Touchpad report carries several contacts plus the
    // contact-count and scan-time trailer and can run past 64 bytes; the
    // length prefix at the head still drives how much of this we parse.
    let mut buf = [0u8; 256];
    // wMaxInputLength is unreliable: some firmwares report 0 or a small value
    // that is shorter than a real multi-contact report, which then fails the
    // length-prefix check below and drops every frame. Always offer the full
    // buffer and let the 2-byte length prefix say how much of it is valid.
    let len = buf.len();
    state.input_polls = state.input_polls.wrapping_add(1);
    // Spec first: after reset the device auto-points at the input register
    // and reports come from a bare read. Devices that only answer a
    // register-addressed read get the fallback.
    let n = match write_read(state.i2c_port, state.addr, &[], &mut buf[..len]) {
        Some(n) if n >= 2 => n,
        _ => {
            let reg = state.input_register.to_le_bytes();
            let Some(n) = write_read(state.i2c_port, state.addr, &reg, &mut buf[..len]) else {
                return;
            };
            n
        }
    };
    if n < 2 {
        return;
    }
    state.input_reports = state.input_reports.wrapping_add(1);
    // A 0x0000 length prefix is the "nothing pending" / reset sentinel, not a
    // report; only count frames that carry data.
    if buf[0] != 0 || buf[1] != 0 {
        signal_raw_report();
    }

    // An absolute touchpad decodes through the parsed field map and the gesture
    // engine; anything else falls back to the relative boot-mouse decode.
    if state.touch_layout.is_absolute_touch() {
        // The report opens with a 2-byte total length covering the prefix and
        // body. Reject a prefix that is too small to hold itself or larger than
        // what was actually read, and skip the poll instead of slicing wild.
        let total = u16::from_le_bytes([buf[0], buf[1]]) as usize;
        if total < 2 || total > n {
            return;
        }
        let body = &buf[2..total];
        if let Some(s) = decode_touch(body, &state.touch_layout) {
            let act =
                state.gesture.on_touch(s.x, s.y, s.x_max, s.y_max, s.tip, s.contacts, s.button);
            publish_touch(state, &act);
            return;
        }
        // In PTP mode every meaningful frame is the touch report; anything
        // else here (vendor report ids, torn reads) must be dropped, not
        // reinterpreted by the boot-mouse heuristic as random ±127 deltas.
        if state.ptp_mode {
            return;
        }
        // A pad that stayed in its power-on mouse mode (the input-mode switch
        // failed or is absent) streams under the mouse collection's report id,
        // which the touch field map rejects; fall through to the relative
        // decode instead of dropping every frame a healthy device sends.
    }
    if let Some(sample) = parse_report(&buf[..n]) {
        publish(state, sample);
    }
}
