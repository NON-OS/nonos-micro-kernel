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

use crate::constants::{GCAP, GCTL, GSTS, INPAY, INTCTL, INTSTS, OUTPAY, STATESTS, VMAJ, VMIN};
use crate::controller::streams;
use crate::regs::Regs;

#[derive(Clone, Copy)]
pub struct ControllerInfo {
    pub gcap: u16,
    pub vmin: u8,
    pub vmaj: u8,
    pub outpay: u16,
    pub inpay: u16,
    pub gctl: u32,
    pub statests: u16,
    pub gsts: u16,
    pub intctl: u32,
    pub intsts: u32,
    pub input_streams: u8,
    pub output_streams: u8,
    pub bidi_streams: u8,
    pub addr64: u8,
}

impl ControllerInfo {
    pub fn read(regs: Regs) -> Self {
        let gcap = unsafe { regs.r16(GCAP) };
        Self {
            gcap,
            vmin: unsafe { regs.r8(VMIN) },
            vmaj: unsafe { regs.r8(VMAJ) },
            outpay: unsafe { regs.r16(OUTPAY) },
            inpay: unsafe { regs.r16(INPAY) },
            gctl: unsafe { regs.r32(GCTL) },
            statests: unsafe { regs.r16(STATESTS) },
            gsts: unsafe { regs.r16(GSTS) },
            intctl: unsafe { regs.r32(INTCTL) },
            intsts: unsafe { regs.r32(INTSTS) },
            input_streams: streams::input_streams(gcap),
            output_streams: streams::output_streams(gcap),
            bidi_streams: streams::bidi_streams(gcap),
            addr64: streams::addr64(gcap),
        }
    }
}
