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

//! Execute a power-sequence table against the register surface. This is the
//! engine every rtw88 MAC power transition runs on: read-modify-write a byte
//! register, poll a byte register for a value, or stop at the end marker. It is
//! pure control flow over the `Mmio` trait, so a host proof drives it with a
//! modeled device and checks the exact register program a table produces, with
//! no hardware.

use super::command::{PwrCmd, CMD_END, CMD_POLL, CMD_WRITE};
use crate::regs::Mmio;

/// Reads allowed before a poll step is declared failed. Real transitions settle
/// in a handful of microseconds; the bound only stops a wedged card hanging the
/// driver.
const POLL_LIMIT: u32 = 1_000_000;

/// Run a power-sequence table. Returns `true` when it reaches the end command,
/// `false` if a poll step never observed its value (a dead or unpowered card).
pub fn run_pwr_seq<M: Mmio>(mmio: &M, table: &[PwrCmd]) -> bool {
    for step in table {
        match step.cmd {
            CMD_WRITE => {
                let cur = mmio.read8(step.offset as usize);
                let next = (cur & !step.mask) | (step.value & step.mask);
                mmio.write8(step.offset as usize, next);
            }
            CMD_POLL => {
                if !poll(mmio, step) {
                    return false;
                }
            }
            CMD_END => return true,
            _ => return false,
        }
    }
    // A table without an explicit end still ran to completion.
    true
}

fn poll<M: Mmio>(mmio: &M, step: &PwrCmd) -> bool {
    let want = step.value & step.mask;
    for _ in 0..POLL_LIMIT {
        if mmio.read8(step.offset as usize) & step.mask == want {
            return true;
        }
        core::hint::spin_loop();
    }
    false
}
