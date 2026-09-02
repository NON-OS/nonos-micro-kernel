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

//! Which of the ring-0 restrictions this part actually implements.
//!
//! A bit absent here is a bit that must not be written: setting a reserved
//! CR4 bit raises #GP, so every write downstream is gated on an answer from
//! this module rather than attempted and hoped for.

use super::super::super::constants::{
    CPUID_EBX_SMAP, CPUID_EBX_SMEP, CPUID_ECX_UMIP, CPUID_EDX_NX, CPUID_EXTENDED_LEAF,
    CPUID_FEATURES_LEAF,
};

#[derive(Clone, Copy)]
pub(super) struct Supported {
    pub smep: bool,
    pub smap: bool,
    pub umip: bool,
    pub nx: bool,
}

pub(super) fn supported() -> Supported {
    let (_, ebx, ecx, _) = read(CPUID_FEATURES_LEAF, 0);
    let (_, _, _, edx) = read(CPUID_EXTENDED_LEAF, 0);
    Supported {
        smep: ebx & CPUID_EBX_SMEP != 0,
        smap: ebx & CPUID_EBX_SMAP != 0,
        umip: ecx & CPUID_ECX_UMIP != 0,
        nx: edx & CPUID_EDX_NX != 0,
    }
}

fn read(leaf: u32, subleaf: u32) -> (u32, u32, u32, u32) {
    let r = core::arch::x86_64::__cpuid_count(leaf, subleaf);
    (r.eax, r.ebx, r.ecx, r.edx)
}
