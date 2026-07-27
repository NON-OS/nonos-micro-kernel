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

use crate::constants::{IC, IR, IRS, IRS_BUSY, IRS_VALID, VERB_GET_PARAMETER};
use crate::error::{HdaError, HdaResult};
use crate::regs::Regs;

const WAIT_SPINS: u32 = 1_000_000;

pub fn get_parameter(regs: Regs, codec: u8, node: u8, param: u16) -> HdaResult<u32> {
    send(regs, compose_verb(codec, node, VERB_GET_PARAMETER, param))
}

fn send(regs: Regs, verb: u32) -> HdaResult<u32> {
    wait_busy_clear(regs)?;
    unsafe {
        regs.w8(IRS, IRS_VALID);
        regs.w32(IC, verb);
        regs.w8(IRS, IRS_BUSY);
    }
    wait_response(regs)
}

fn wait_busy_clear(regs: Regs) -> HdaResult<()> {
    let mut spins = 0u32;
    while spins < WAIT_SPINS {
        if unsafe { regs.r8(IRS) } & IRS_BUSY == 0 {
            return Ok(());
        }
        spins = spins.wrapping_add(1);
        core::hint::spin_loop();
    }
    Err(HdaError::ImmediateCommandBusy)
}

fn wait_response(regs: Regs) -> HdaResult<u32> {
    let mut spins = 0u32;
    while spins < WAIT_SPINS {
        let status = unsafe { regs.r8(IRS) };
        if status & IRS_BUSY == 0 && status & IRS_VALID != 0 {
            return Ok(unsafe { regs.r32(IR) });
        }
        spins = spins.wrapping_add(1);
        core::hint::spin_loop();
    }
    Err(HdaError::ImmediateResponseTimeout)
}

pub(crate) const fn compose_verb(codec: u8, node: u8, verb: u16, payload: u16) -> u32 {
    ((codec as u32 & 0x0f) << 28)
        | ((node as u32 & 0x7f) << 20)
        | ((verb as u32 & 0x0fff) << 8)
        | (payload as u32 & 0xff)
}

pub(crate) const fn compose_verb_long(cad: u8, nid: u8, verb4: u16, payload16: u16) -> u32 {
    ((cad as u32 & 0x0f) << 28)
        | ((nid as u32 & 0x7f) << 20)
        | ((verb4 as u32 & 0x0f) << 16)
        | (payload16 as u32)
}
