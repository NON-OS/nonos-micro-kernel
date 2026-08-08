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

pub fn read_aa64isar0() -> u64 {
    let value: u64;
    unsafe {
        asm!("mrs {}, id_aa64isar0_el1", out(reg) value, options(nostack));
    }
    value
}

pub fn read_aa64isar1() -> u64 {
    let value: u64;
    unsafe {
        asm!("mrs {}, id_aa64isar1_el1", out(reg) value, options(nostack));
    }
    value
}

pub fn read_aa64pfr0() -> u64 {
    let value: u64;
    unsafe {
        asm!("mrs {}, id_aa64pfr0_el1", out(reg) value, options(nostack));
    }
    value
}

pub fn read_aa64pfr1() -> u64 {
    let value: u64;
    unsafe {
        asm!("mrs {}, id_aa64pfr1_el1", out(reg) value, options(nostack));
    }
    value
}

/// `ID_AA64ZFR0_EL1`, which reports what the scalable vector extension supports.
///
/// Named by its encoding rather than its mnemonic on purpose: an assembler only
/// accepts `id_aa64zfr0_el1` when the target carries SVE, and demanding that
/// would mean either refusing to build for parts without it or handing the
/// compiler a feature bit it may then emit into ordinary code. The encoding is
/// architecturally fixed and reads as zero where SVE is absent, which is exactly
/// the answer a feature probe wants.
pub fn read_aa64zfr0() -> u64 {
    let value: u64;
    // SAFETY: S3_0_C0_C4_4 is ID_AA64ZFR0_EL1. Reading an ID register at EL1 has
    // no side effects.
    unsafe {
        asm!("mrs {}, s3_0_c0_c4_4", out(reg) value, options(nostack));
    }
    value
}

/// `ID_AA64MMFR1_EL1`, which reports the memory-model features PAN lives in.
pub fn read_aa64mmfr1() -> u64 {
    let value: u64;
    // SAFETY: an identification register, always readable at EL1, no side effects.
    unsafe {
        asm!("mrs {}, id_aa64mmfr1_el1", out(reg) value, options(nomem, nostack, preserves_flags));
    }
    value
}
