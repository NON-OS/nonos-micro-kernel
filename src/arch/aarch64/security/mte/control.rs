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

use core::arch::asm;

use crate::arch::aarch64::cpu::features::{has_feature, CpuFeature};

use super::mode::MteMode;

pub fn set_mte_mode(mode: MteMode) {
    if has_feature(CpuFeature::Mte) {
        let tcf = mode.tcf();
        let mut sctlr = read_sctlr();
        sctlr &= !(0b11 << 40);
        sctlr |= tcf << 40;
        sctlr &= !(0b11 << 38);
        sctlr |= tcf << 38;
        write_sctlr(sctlr);
    }
}

pub fn mte_enabled() -> bool {
    has_feature(CpuFeature::Mte)
}

fn read_sctlr() -> u64 {
    let sctlr: u64;
    unsafe {
        asm!("mrs {}, sctlr_el1", out(reg) sctlr);
    }
    sctlr
}

fn write_sctlr(sctlr: u64) {
    unsafe {
        asm!("msr sctlr_el1, {}", "isb", in(reg) sctlr);
    }
}
