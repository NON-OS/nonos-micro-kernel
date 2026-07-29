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

//! Which CPU is executing, read out of MPIDR_EL1.
//!
//! Two different questions get asked of MPIDR and they need different answers.
//! Hardware that routes interrupts wants the full affinity, byte per level, in
//! the layout the GIC compares against. Kernel bookkeeping wants a small dense
//! number to index an array with. [`cpu_affinity`] answers the first;
//! [`cpu_id`] answers the second.

use super::mpidr::mpidr;

/// This CPU's affinity, in the layout the GIC compares against.
pub fn cpu_affinity() -> u32 {
    pack_affinity(mpidr())
}

/// The four affinity levels of an MPIDR-shaped value, packed one byte each as
/// `Aff3:Aff2:Aff1:Aff0`.
///
/// MPIDR_EL1 keeps Aff3 up at bits 39:32 with the other three in the low word,
/// and the device tree's `cpu/reg` uses the same shape. GICR_TYPER instead
/// reports the four levels contiguously, so a redistributor belongs to a CPU
/// when this packing of the CPU's affinity equals what that register says.
pub const fn pack_affinity(mpidr_affinity: u64) -> u32 {
    let aff0 = (mpidr_affinity & 0xFF) as u32;
    let aff1 = ((mpidr_affinity >> 8) & 0xFF) as u32;
    let aff2 = ((mpidr_affinity >> 16) & 0xFF) as u32;
    let aff3 = ((mpidr_affinity >> 32) & 0xFF) as u32;
    (aff3 << 24) | (aff2 << 16) | (aff1 << 8) | aff0
}

/// This CPU's MPIDR affinity with the non-affinity bits masked off: the value
/// the device tree publishes and PSCI expects as a target.
pub fn mpidr_affinity() -> u64 {
    mpidr() & 0x0000_00FF_00FF_FFFF
}

/// This CPU's dense index: its position in the device tree's CPU list.
///
/// Contiguous by construction, which is what per-CPU tables need. Falls back
/// to zero before the roster is latched, which is correct because only the
/// boot CPU is running then. It is not an affinity: never hand it to the GIC
/// or to PSCI, which both want [`cpu_affinity`] or [`mpidr_affinity`].
pub fn cpu_id() -> usize {
    crate::arch::aarch64::boot::multicore::roster::index_of(mpidr_affinity()).unwrap_or(0)
}

pub fn core_id() -> usize {
    (mpidr() & 0xFF) as usize
}

pub fn cluster_id() -> usize {
    ((mpidr() >> 8) & 0xFF) as usize
}

pub fn affinity_level(level: u32) -> u64 {
    let value = mpidr();
    match level {
        0 => value & 0xFF,
        1 => (value >> 8) & 0xFF,
        2 => (value >> 16) & 0xFF,
        3 => (value >> 32) & 0xFF,
        _ => 0,
    }
}

pub fn is_primary_core() -> bool {
    cpu_id() == 0
}

pub fn is_multiprocessor() -> bool {
    (mpidr() & (1 << 30)) == 0
}
