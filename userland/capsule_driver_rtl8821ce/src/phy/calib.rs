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

//! Run the RF IQ calibration. The driver asks the firmware to calibrate by
//! sending an IQK H2C packet, then waits for the 8051 to report completion by
//! writing a fixed marker into an RF register, and finally clears that marker.
//! The calibration itself runs on the 8051, so only the request, the
//! completion-wait and the clear are on the host; the packet transport is
//! injected so this reads against a modeled device. This is rtw88
//! `rtw8821c_do_iqk` in `rtw8821c.c`; the flow is checked in `rtl8821ce_proofs`.

use super::rf;
use crate::h2c::build_iqk;
use crate::regs::Mmio;

/// `RF_DTXLOK`: the RF register the firmware writes the completion marker into.
const RF_DTXLOK: u8 = 0x08;
/// The value the firmware writes once IQK has finished.
const IQK_DONE: u32 = 0x0A_BCDE;
/// Reads allowed before calibration is declared not to have completed.
const IQK_POLL_LIMIT: u32 = 300_000;

/// Trigger IQK and wait for it to complete. `send` transmits the H2C packet down
/// the H2C queue and reports whether it was accepted; `seq` is the H2C sequence
/// number and `segment` runs a segmented calibration (used while associated).
/// Returns false if the packet is refused or the firmware never signals done.
pub fn iqk<M, F>(mmio: &M, mut send: F, seq: u8, segment: bool) -> bool
where
    M: Mmio,
    F: FnMut(&[u8]) -> bool,
{
    let pkt = build_iqk(false, segment, seq);
    if !send(&pkt) {
        return false;
    }
    let done = wait_done(mmio);
    // Clear the marker whether or not it completed, so the next run starts clean.
    rf::write_a(mmio, RF_DTXLOK, 0);
    done
}

// The firmware writes IQK_DONE into RF_DTXLOK when the 8051 finishes calibrating.
fn wait_done<M: Mmio>(mmio: &M) -> bool {
    for _ in 0..IQK_POLL_LIMIT {
        if rf::read_a(mmio, RF_DTXLOK) == IQK_DONE {
            return true;
        }
        core::hint::spin_loop();
    }
    false
}
