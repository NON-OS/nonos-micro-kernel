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

use nonos_libc::{mk_input_event_post, InputEvent};

// Diagnostic sentinel (kernel diag slot 6, rendered as an on-screen bar): a
// non-empty report arrived from the pad, counted before any decode. A photo
// then separates "the pad sends nothing" (bar flat while dragging) from
// "reports arrive but decode drops them" (bar grows, events bar flat).
const KIND_RAW_REPORT: u16 = 0xFE06;

pub(super) fn signal_raw_report() {
    let ev = InputEvent {
        kind: KIND_RAW_REPORT,
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
