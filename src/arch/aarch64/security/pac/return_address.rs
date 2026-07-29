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

//! Signing and authenticating a saved return address with the IA key.
//!
//! `PACIASP` cannot express this: it signs whatever is in LR against whatever
//! is in SP, and inline asm cannot hand the assembler a stack pointer operand.
//! The general forms `PACIA`/`AUTIA` take both the value and the modifier in
//! ordinary registers, which is what a kernel saving a frame elsewhere needs.
//!
//! Both instructions are only defined when the core implements FEAT_PAuth, and
//! the assembler only encodes them when the `pauth` extension is enabled, so
//! each block turns the extension on for its own scope. When the feature is
//! absent the address passes through unchanged: an unsigned pointer is exactly
//! what authenticates on a core with no signing, which is the same behaviour
//! `PACIASP` has there as a `NOP`.

use core::arch::asm;

use crate::arch::aarch64::cpu::features::{has_feature, CpuFeature};

pub fn sign_return_address(lr: u64, sp: u64) -> u64 {
    if !has_feature(CpuFeature::Pauth) {
        return lr;
    }
    let mut signed = lr;
    // SAFETY: FEAT_PAuth is implemented, so PACIA is defined at EL1. It writes
    // only its destination register, reads no memory and needs no stack.
    unsafe {
        asm!(
            ".arch_extension pauth",
            "pacia {value}, {modifier}",
            ".arch_extension nopauth",
            value = inout(reg) signed,
            modifier = in(reg) sp,
            options(nomem, nostack, preserves_flags),
        );
    }
    signed
}

pub fn authenticate_return_address(lr: u64, sp: u64) -> u64 {
    if !has_feature(CpuFeature::Pauth) {
        return lr;
    }
    let mut authenticated = lr;
    // SAFETY: as for `sign_return_address`. A failed authentication does not
    // fault here; it poisons the pointer, and the branch that uses it is what
    // traps.
    unsafe {
        asm!(
            ".arch_extension pauth",
            "autia {value}, {modifier}",
            ".arch_extension nopauth",
            value = inout(reg) authenticated,
            modifier = in(reg) sp,
            options(nomem, nostack, preserves_flags),
        );
    }
    authenticated
}
