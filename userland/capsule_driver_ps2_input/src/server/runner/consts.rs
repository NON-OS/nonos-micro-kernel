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
pub(super) const POLL_IDLE_MS: u64 = 1;
pub(super) const IRQ_WAIT_MS: u64 = 100;

/* driver-ready sentinel: bring-up diagnostic, silenced
const KIND_DRIVER_READY: u16 = 0xFE01;

fn signal_ready() {
    let ev = InputEvent {
        kind: KIND_DRIVER_READY,
        flags: 0,
        code: 0,
        x: 0,
        y: 0,
        delta_x: 0,
        delta_y: 0,
        timestamp_ns: 0,
    };
    let _ = mk_input_event_post(&ev);
}
*/
