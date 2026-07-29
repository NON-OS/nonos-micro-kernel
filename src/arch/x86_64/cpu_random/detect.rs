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

//! CPUID feature bits for the two on-die generators.

use crate::arch::x86_64::cpu::cpuid::{cpuid, cpuid_count};

const LEAF_FEATURES: u32 = 1;
const LEAF_EXTENDED: u32 = 7;
const ECX_RDRAND: u32 = 1 << 30;
const EBX_RDSEED: u32 = 1 << 18;

/// CPUID.01H:ECX[30].
pub fn has_rdrand() -> bool {
    let (_, _, ecx, _) = cpuid(LEAF_FEATURES);
    ecx & ECX_RDRAND != 0
}

/// CPUID.07H:EBX[18].
pub fn has_rdseed() -> bool {
    let (_, ebx, _, _) = cpuid_count(LEAF_EXTENDED, 0);
    ebx & EBX_RDSEED != 0
}
