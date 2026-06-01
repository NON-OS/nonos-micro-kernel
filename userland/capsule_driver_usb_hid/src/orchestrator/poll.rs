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

use nonos_libc::mk_yield;

use crate::descriptors::HidKind;
use crate::state::State;
use crate::xhci::interrupt_in;

use super::enumerate::{enumerate, HidEndpoint};

const HID_REPORT_MAX: usize = 8;

pub fn run(xhci_port: u32) -> ! {
    let eps = enumerate(xhci_port);
    let mut state = State::new();
    let mut buf = [0u8; HID_REPORT_MAX];
    loop {
        for ep in &eps {
            drain_endpoint(&mut state, ep, &mut buf);
        }
        mk_yield();
    }
}

fn drain_endpoint(state: &mut State, ep: &HidEndpoint, buf: &mut [u8; HID_REPORT_MAX]) {
    loop {
        match interrupt_in(ep.port, ep.slot, ep.dci, ep.max_packet, buf) {
            Ok(Some(n)) => feed_report(state, ep, buf, n),
            _ => return,
        }
    }
}

fn feed_report(state: &mut State, ep: &HidEndpoint, buf: &[u8; HID_REPORT_MAX], n: usize) {
    match ep.kind {
        HidKind::Keyboard => {
            let mut report8 = [0u8; 8];
            let copy_len = n.min(8);
            report8[..copy_len].copy_from_slice(&buf[..copy_len]);
            state.keyboard.feed(&report8);
        }
        HidKind::Mouse => {
            state.mouse.feed(&buf[..n]);
        }
    }
}
