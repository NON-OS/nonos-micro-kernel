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

use super::constants::MAX_CPUS;
use super::types::CpuDescriptor;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};

pub(crate) static CPU_DESCRIPTORS: [CpuDescriptor; MAX_CPUS] = {
    const INIT: CpuDescriptor = CpuDescriptor::new();
    [INIT; MAX_CPUS]
};

pub(super) static CPU_COUNT: AtomicUsize = AtomicUsize::new(1);

pub(super) static CPUS_ONLINE: AtomicUsize = AtomicUsize::new(1);

pub(super) static BSP_APIC_ID: AtomicU32 = AtomicU32::new(0);

pub(super) static SMP_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub(super) static BSP_INITIALIZING: AtomicBool = AtomicBool::new(false);

pub(super) static AP_STARTUP_BARRIER: AtomicU32 = AtomicU32::new(0);

pub(crate) fn cpu_count() -> usize {
    CPU_COUNT.load(Ordering::Acquire)
}

pub(crate) fn cpus_online() -> usize {
    CPUS_ONLINE.load(Ordering::Acquire)
}
