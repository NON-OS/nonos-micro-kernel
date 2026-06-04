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

use crate::orchestrator::enumerate::enumerate;
use crate::protocol::{HDR_LEN, IPC_PAYLOAD_MAX};
use crate::server::pump_once;
use crate::state::State;

use super::constants::{HID_REPORT_MAX, RESCAN_INTERVAL};
use super::drain_endpoints::drain_endpoints;
use super::refresh_endpoints::refresh_endpoints;

pub fn run(xhci_port: u32) -> ! {
    let mut eps = enumerate(xhci_port);
    let mut state = State::new();
    let mut buf = [0u8; HID_REPORT_MAX];
    let mut rx = alloc::vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = alloc::vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut idle_polls = 0u32;
    loop {
        let _ = pump_once(&mut state, &mut rx, &mut tx);
        if drain_endpoints(&mut state, &eps, &mut buf) {
            idle_polls = 0;
        } else if eps.is_empty() {
            idle_polls = idle_polls.saturating_add(1);
        }
        if eps.is_empty() && idle_polls >= RESCAN_INTERVAL {
            refresh_endpoints(xhci_port, &mut eps);
            idle_polls = 0;
        }
        mk_yield();
    }
}
