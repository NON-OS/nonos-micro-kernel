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

use crate::constants::{
    CORBCTL, CORBCTL_RUN, CORBLBASE, CORBRP, CORBRP_RST, CORBSIZE, CORBSIZE_256, CORBUBASE, CORBWP,
    RINTCNT, RINTCNT_ONE, RIRBCTL, RIRBCTL_DMAEN, RIRBLBASE, RIRBSIZE, RIRBSIZE_256, RIRBUBASE,
    RIRBWP, RIRBWP_RST,
};
use crate::error::{HdaError, HdaResult};
use crate::regs::Regs;

const RESET_SPINS: u32 = 1_000_000;

pub fn init(regs: Regs, corb_pa: u64, rirb_pa: u64) -> HdaResult<()> {
    unsafe {
        regs.w8(CORBCTL, 0);
        regs.w8(RIRBCTL, 0);
        regs.w8(CORBSIZE, CORBSIZE_256);
        regs.w8(RIRBSIZE, RIRBSIZE_256);
        regs.w32(CORBLBASE, corb_pa as u32);
        regs.w32(CORBUBASE, (corb_pa >> 32) as u32);
        regs.w32(RIRBLBASE, rirb_pa as u32);
        regs.w32(RIRBUBASE, (rirb_pa >> 32) as u32);
    }
    reset_read_pointer(regs)?;
    unsafe {
        regs.w16(CORBWP, 0);
        regs.w16(RIRBWP, RIRBWP_RST);
        regs.w16(RINTCNT, RINTCNT_ONE);
        regs.w8(CORBCTL, CORBCTL_RUN);
        regs.w8(RIRBCTL, RIRBCTL_DMAEN);
    }
    Ok(())
}

fn reset_read_pointer(regs: Regs) -> HdaResult<()> {
    unsafe { regs.w16(CORBRP, CORBRP_RST) };
    wait_corbrp(regs, true)?;
    unsafe { regs.w16(CORBRP, 0) };
    wait_corbrp(regs, false)
}

fn wait_corbrp(regs: Regs, want_set: bool) -> HdaResult<()> {
    let mut spins = 0u32;
    while spins < RESET_SPINS {
        let set = unsafe { regs.r16(CORBRP) } & CORBRP_RST != 0;
        if set == want_set {
            return Ok(());
        }
        spins = spins.wrapping_add(1);
    }
    Err(HdaError::ControllerResetTimeout)
}
