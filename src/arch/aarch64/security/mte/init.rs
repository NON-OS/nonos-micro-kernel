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

pub fn init_mte() {
    // Everything below is EL1 state: the TCR tag-checking bits, SCTLR.ATA and
    // GCR_EL1. All three arrive with FEAT_MTE2, so a part reporting plain MTE,
    // which is EL0 instructions and nothing else, must not come through here.
    if has_feature(CpuFeature::Mte2) {
        configure_tcr_mte();
        configure_sctlr_mte();
        configure_gcr();
    }
}

fn configure_tcr_mte() {
    let tcr = read_tcr() | (1 << 37) | (1 << 38);
    unsafe {
        asm!("msr tcr_el1, {}", "isb", in(reg) tcr);
    }
}

fn configure_sctlr_mte() {
    let sctlr = read_sctlr() | (1 << 43);
    unsafe {
        asm!("msr sctlr_el1, {}", "isb", in(reg) sctlr);
    }
}

fn configure_gcr() {
    let gcr: u64 = 0xFFFF << 1;
    unsafe {
        asm!("msr gcr_el1, {}", "isb", in(reg) gcr);
    }
}

fn read_tcr() -> u64 {
    let tcr: u64;
    unsafe {
        asm!("mrs {}, tcr_el1", out(reg) tcr);
    }
    tcr
}

fn read_sctlr() -> u64 {
    let sctlr: u64;
    unsafe {
        asm!("mrs {}, sctlr_el1", out(reg) sctlr);
    }
    sctlr
}
