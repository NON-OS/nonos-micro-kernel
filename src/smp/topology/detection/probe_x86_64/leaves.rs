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

//! Counting cores and threads out of the CPUID topology leaves.

use super::super::super::types::CpuTopology;

#[inline]
pub(super) fn cpuid(leaf: u32, subleaf: u32) -> (u32, u32, u32, u32) {
    // CPUID is unprivileged and has no side effects. Leaves above the reported
    // maximum return zero rather than faulting, and callers check it.
    let r = core::arch::x86_64::__cpuid_count(leaf, subleaf);
    (r.eax, r.ebx, r.ecx, r.edx)
}

/// Leaf 0x0B, the extended topology enumeration: one subleaf per level, each
/// naming its type and how many processors it spans.
pub(super) fn via_leaf_0b(topology: &mut CpuTopology) -> usize {
    let mut threads = 0usize;
    let mut cores = 0usize;

    for subleaf in 0..3 {
        let (_, ebx, ecx, _) = cpuid(0x0B, subleaf);
        let level_type = (ecx >> 8) & 0xFF;
        let processors = (ebx & 0xFFFF) as usize;
        match level_type {
            0 => break,
            1 => threads = processors,
            2 => cores = processors,
            _ => {}
        }
    }

    if cores > 0 {
        topology.physical_cores = cores;
    }
    if threads > 0 {
        topology.hyperthreading = threads > cores;
        return threads;
    }
    1
}

/// Leaf 4, the deterministic cache parameters, which also carries the maximum
/// core count per package. Used on parts predating leaf 0x0B.
pub(super) fn via_leaf_04(topology: &mut CpuTopology) -> usize {
    let (eax, _, _, _) = cpuid(4, 0);
    let max_cores = (((eax >> 26) & 0x3F) + 1) as usize;
    topology.physical_cores = max_cores;

    let (_, ebx, _, _) = cpuid(1, 0);
    let logical = ((ebx >> 16) & 0xFF) as usize;
    if logical > 0 {
        topology.hyperthreading = logical > max_cores;
        return logical;
    }
    max_cores
}
