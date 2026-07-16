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

use super::parse_report::parse_report;
use super::publish::publish;
use super::publish_touch::publish_touch;
use crate::hid::decode_touch;
use crate::i2c_client::write_read;
use crate::state::State;

// Diagnostic sentinel (kernel diag slot 6, rendered as an on-screen bar): a
// non-empty report arrived from the pad, counted before any decode. A photo
// then separates "the pad sends nothing" (bar flat while dragging) from
// "reports arrive but decode drops them" (bar grows, events bar flat).
const KIND_RAW_REPORT: u16 = 0xFE06;

// How many quiet polls (~2ms each) after the last decoded touch frame before
// the boot-mouse fallback reopens for unmatched report ids.
const TOUCH_GATE_POLLS: u32 = 1024;

// Verbatim-repeat handling: a device generates reports at its own rate
// (~8ms) while we poll faster, so an identical frame is processed only every
// PACE-th poll, and a run longer than MAX repeats is a stale buffer (the pad
// re-serving its last report), not input. Real finger motion changes the
// bytes constantly and is untouched by either bound.
const FRAME_REPEAT_PACE: u32 = 4;
const FRAME_REPEAT_MAX: u32 = 12;

fn signal_raw_report() {
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

pub fn poll(state: &mut State) {
    if !state.found() || state.input_register == 0 {
        return;
    }
    // A full Precision Touchpad report carries several contacts plus the
    // contact-count and scan-time trailer and can run past 64 bytes; the
    // length prefix at the head still drives how much of this we parse.
    let mut buf = [0u8; 256];
    // Read only as much as the device declares (with a floor for firmwares
    // that under-report wMaxInputLength). The read length is transfer TIME on
    // the wire: a 256-byte read at fast-mode speed takes longer than the
    // pad's report period, so it straddles a device-side report update and
    // tears the coordinates mid-transfer. Keeping the read within the real
    // report size keeps it inside one report interval.
    let declared = state.input_len;
    let len = if declared >= 5 { declared.min(buf.len()) } else { 64 };
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
        // The first few raw frames go to the boot console so a photograph
        // shows the exact wire bytes next to the parsed layout.
        if state.frame_dumps < 6 {
            state.frame_dumps += 1;
            let tag = alloc::format!("[i2chid] frm{} n={}:", state.frame_dumps, n);
            crate::diag::hex_line(&tag, &buf[..n.min(16)]);
        }
    }
    // Stale-frame suppression: polled reads return the current report whether
    // or not it is new. A byte-identical frame is paced down to the device's
    // own report rate, and a long verbatim run is the pad re-serving a stale
    // buffer (its last motion report after finger lift, or a post-reset
    // announcement); treating those as input drifts the cursor by itself.
    let snap = n.min(state.last_frame.len());
    let identical = state.last_frame_len == n && state.last_frame[..snap] == buf[..snap];
    if identical {
        state.frame_repeats = state.frame_repeats.saturating_add(1);
        if state.frame_repeats > FRAME_REPEAT_MAX
            || !state.frame_repeats.is_multiple_of(FRAME_REPEAT_PACE)
        {
            return;
        }
    } else {
        state.last_frame[..snap].copy_from_slice(&buf[..snap]);
        state.last_frame_len = n;
        state.frame_repeats = 0;
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
            state.touch_decoded = true;
            state.polls_since_touch = 0;
            let act = state.gesture.on_touch(
                s.x,
                s.y,
                s.x_max,
                s.y_max,
                s.tip,
                s.confidence,
                s.contacts,
                s.button,
            );
            publish_touch(state, &act);
            return;
        }
        // While the touch decoder is proving itself on this device, unmatched
        // frames (vendor report ids, torn reads) are dropped rather than
        // reinterpreted by the boot-mouse heuristic as random ±127 deltas.
        // The gate decays: if the absolute stream goes quiet (mode changed,
        // device reset to its mouse collection), the relative fallback
        // reopens instead of muting the pad forever.
        state.polls_since_touch = state.polls_since_touch.saturating_add(1);
        if state.touch_decoded && state.polls_since_touch < TOUCH_GATE_POLLS {
            return;
        }
    }
    if let Some(sample) = parse_report(&buf[..n]) {
        publish(state, sample);
    }
}
