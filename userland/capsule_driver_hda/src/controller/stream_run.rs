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

use super::StreamDescriptor;
use crate::constants::{
    INTCTL, INTCTL_GIE, SDCTL_IOCE, SDCTL_RUN, SDCTL_SRST, SDSTS_BCIS, SD_BDPL, SD_BDPU, SD_CBL,
    SD_CTL, SD_FMT, SD_LVI, SD_STS, STREAM_FMT_48K16S,
};
use crate::regs::Regs;
use core::ptr::write_volatile;

const RESET_SPINS: u32 = 65_536;

pub(crate) struct StreamRun {
    pub desc: StreamDescriptor,
    pub tag: u8,
    pub bdl_va: u64,
    pub bdl_dev: u64,
    pub sample_dev: u64,
    pub bytes: u32,
}

fn write_bdl(va: u64, addr: u64, len: u32) {
    unsafe {
        write_volatile(va as *mut u64, addr);
        write_volatile((va + 8) as *mut u32, len);
        write_volatile((va + 12) as *mut u32, 1);
    }
}

fn reset(regs: Regs, off: u32) {
    unsafe {
        regs.w8(off + SD_CTL, SDCTL_SRST);
        let mut s = 0u32;
        while regs.r8(off + SD_CTL) & SDCTL_SRST == 0 && s < RESET_SPINS {
            s += 1;
        }
        regs.w8(off + SD_CTL, 0);
        s = 0;
        while regs.r8(off + SD_CTL) & SDCTL_SRST != 0 && s < RESET_SPINS {
            s += 1;
        }
    }
}

pub(crate) fn run(regs: Regs, r: StreamRun) {
    write_bdl(r.bdl_va, r.sample_dev, r.bytes);
    let off = r.desc.mmio_offset;
    reset(regs, off);
    unsafe {
        regs.w8(off + SD_CTL + 2, r.tag << 4);
        regs.w32(off + SD_CBL, r.bytes);
        regs.w16(off + SD_LVI, 0);
        regs.w16(off + SD_FMT, STREAM_FMT_48K16S);
        regs.w32(off + SD_BDPL, r.bdl_dev as u32);
        regs.w32(off + SD_BDPU, (r.bdl_dev >> 32) as u32);
        regs.w8(off + SD_STS, SDSTS_BCIS);
        let ctl = regs.r32(INTCTL) | INTCTL_GIE | (1u32 << r.desc.global_index);
        regs.w32(INTCTL, ctl);
        regs.w8(off + SD_CTL, SDCTL_RUN | SDCTL_IOCE);
    }
}
