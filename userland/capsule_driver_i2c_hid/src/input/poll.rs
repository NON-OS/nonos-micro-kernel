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

use super::parse_report::parse_report;
use super::publish::publish;
use super::publish_touch::publish_touch;
use crate::hid::decode_touch;
use crate::i2c_client::write_read;
use crate::state::State;

pub fn poll(state: &mut State) {
    if !state.found() || state.input_register == 0 || state.input_len < 5 {
        return;
    }
    let mut buf = [0u8; 64];
    let len = state.input_len.min(buf.len());
    let reg = state.input_register.to_le_bytes();
    state.input_polls = state.input_polls.wrapping_add(1);
    let Some(n) = write_read(state.i2c_port, state.addr, &reg, &mut buf[..len]) else { return };
    if n < 2 {
        return;
    }
    state.input_reports = state.input_reports.wrapping_add(1);

    // An absolute touchpad decodes through the parsed field map and the gesture
    // engine; anything else falls back to the relative boot-mouse decode.
    if state.touch_layout.is_absolute_touch() {
        let total = u16::from_le_bytes([buf[0], buf[1]]) as usize;
        let body = &buf[2..total.min(n)];
        if let Some(s) = decode_touch(body, &state.touch_layout) {
            let act =
                state.gesture.on_touch(s.x, s.y, s.x_max, s.y_max, s.tip, s.contacts, s.button);
            publish_touch(state, &act);
        }
    } else if let Some(sample) = parse_report(&buf[..n]) {
        publish(state, sample);
    }
}
