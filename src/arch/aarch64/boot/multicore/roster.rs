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

//! The boot CPU list, in device-tree order.
//!
//! Three things need a CPU's MPIDR affinity rather than a counter: PSCI wants
//! it to name the core to power on, the GIC wants it to match a redistributor
//! frame, and a secondary needs it to find out which entry of the list it is.
//! The device tree is the only place that knows, so the affinities are latched
//! here once, on the boot CPU, before any secondary is released.
//!
//! Position in this list is the kernel's dense CPU index: it is contiguous by
//! construction, which is what per-CPU tables need and what an affinity does
//! not give you.

use core::sync::atomic::{AtomicU32, Ordering};

use super::super::stack::MAX_CPUS;

/// Written once by the boot CPU during `populate`, before any secondary runs.
static mut AFFINITIES: [u64; MAX_CPUS] = [0; MAX_CPUS];
static COUNT: AtomicU32 = AtomicU32::new(0);

/// Record the affinities the device tree reported, in its order.
///
/// An empty list means the device tree could not be read. The boot CPU is
/// still running, so the roster becomes that one CPU rather than nothing: an
/// empty roster would make every dense index a fallback.
///
/// # Safety
///
/// Must be called on the boot CPU before any secondary is started, and only
/// once. Nothing else may be reading the roster at the time.
pub(in crate::arch::aarch64) unsafe fn populate(affinities: &[u64]) {
    let own = [crate::arch::aarch64::cpu::mpidr_affinity()];
    let source = if affinities.is_empty() { &own[..] } else { affinities };
    let n = source.len().min(MAX_CPUS);
    // SAFETY: the caller guarantees exclusive access, so writing the array is
    // not a data race, and `n` is clamped to both lengths.
    unsafe {
        for (slot, affinity) in AFFINITIES.iter_mut().take(n).zip(source) {
            *slot = *affinity;
        }
    }
    COUNT.store(n as u32, Ordering::Release);
}

/// How many CPUs the device tree listed.
pub fn len() -> usize {
    COUNT.load(Ordering::Acquire) as usize
}

/// The affinity of the CPU at dense index `index`.
pub fn affinity_of(index: usize) -> Option<u64> {
    if index >= len() {
        return None;
    }
    // SAFETY: the roster is written once before any reader exists and the
    // index is inside the populated prefix, so this read races with nothing.
    Some(unsafe { AFFINITIES[index] })
}

/// The dense index of the CPU with `affinity`, or `None` if the device tree
/// never mentioned it.
pub fn index_of(affinity: u64) -> Option<usize> {
    (0..len()).find(|i| affinity_of(*i) == Some(affinity))
}
