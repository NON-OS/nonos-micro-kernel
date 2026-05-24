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

pub fn init_bti() {
    if has_feature(CpuFeature::Bti) {
        enable_bti();
    }
}

pub fn enable_bti() {
    if has_feature(CpuFeature::Bti) {
        update_sctlr((1 << 35) | (1 << 36), 0);
    }
}

pub fn disable_bti() {
    update_sctlr(0, (1 << 35) | (1 << 36));
}

pub fn bti_enabled() -> bool {
    has_feature(CpuFeature::Bti) && (read_sctlr() & (1 << 35)) != 0
}

pub fn bti_supported() -> bool {
    has_feature(CpuFeature::Bti)
}

fn update_sctlr(set: u64, clear: u64) {
    let sctlr = (read_sctlr() | set) & !clear;
    unsafe {
        asm!("msr sctlr_el1, {}", "isb", in(reg) sctlr);
    }
}

fn read_sctlr() -> u64 {
    let sctlr: u64;
    unsafe {
        asm!("mrs {}, sctlr_el1", out(reg) sctlr);
    }
    sctlr
}
