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

use core::ptr::{read_volatile, write_volatile};

use crate::constants::{CORBWP, RIRBWP};
use crate::error::{HdaError, HdaResult};
use crate::regs::Regs;

const RING_MASK: u16 = 0xff;
const VERB_SPINS: u32 = 1_000_000;

pub fn send(regs: Regs, corb_va: u64, rirb_va: u64, wp: &mut u16, cmd: u32) -> HdaResult<u32> {
    let next = (*wp + 1) & RING_MASK;
    unsafe {
        write_volatile((corb_va + next as u64 * 4) as *mut u32, cmd);
        regs.w16(CORBWP, next);
    }
    wait_rirb(regs, next)?;
    let resp = unsafe { read_volatile((rirb_va + next as u64 * 8) as *const u32) };
    *wp = next;
    Ok(resp)
}

fn wait_rirb(regs: Regs, target: u16) -> HdaResult<()> {
    let mut spins = 0u32;
    while spins < VERB_SPINS {
        if unsafe { regs.r16(RIRBWP) } & RING_MASK == target {
            return Ok(());
        }
        spins = spins.wrapping_add(1);
        core::hint::spin_loop();
    }
    Err(HdaError::VerbTimeout)
}
