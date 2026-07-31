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

//! Turning what the device tree said into the neutral handoff fields.

use super::super::cpu::CpuTopology;
use super::super::measurement::Measurement;
use super::super::memory::MemoryHandoff;
use super::super::timing::TimingHandoff;
use crate::arch::aarch64::boot::info::{BootInfo, MemoryType};

pub(super) fn memory(info: &BootInfo) -> MemoryHandoff {
    let largest_usable_bytes = info
        .memory_map()
        .iter()
        .filter(|r| r.region_type == MemoryType::Available)
        .map(|r| r.size)
        .max()
        .unwrap_or(info.ram_size);

    MemoryHandoff {
        // Nothing walks this pointer blind: the only code that reads the
        // aarch64 memory map goes through the arch downcast, which hands it the
        // typed `BootInfo`. The pointer and count are here so a diagnostic dump
        // can report where the map came from.
        map_ptr: info.memory_map().as_ptr() as u64,
        map_entries: info.memory_map().len() as u32,
        largest_usable_bytes,
    }
}

pub(super) fn cpus(info: &BootInfo) -> CpuTopology {
    CpuTopology { boot_cpu_id: 0, cpu_count: info.cpu_count }
}

pub(super) fn timing() -> TimingHandoff {
    // CNTFRQ_EL0 is the generic timer's tick rate, and the clock needs it to
    // turn a counter delta into milliseconds. Leaving it unset stops the clock
    // advancing at all, which does not fail loudly: anything waiting on elapsed
    // time waits forever, and the boot simply stops making progress.
    //
    // Zero means firmware never told the part its own frequency and there is
    // nothing to infer from, so `None` stays the honest answer there. The device
    // tree carries no wall-clock time either way; the RTC supplies the epoch.
    let hz = crate::arch::aarch64::timer::frequency();
    TimingHandoff { fixed_freq_hz: (hz > 0).then_some(hz), unix_epoch_ms: 0 }
}

pub(super) fn measurement() -> Measurement {
    // Nothing in this boot path has verified a signature: the aarch64
    // bootloader chain does not exist yet. Claiming otherwise would put a
    // measurement in the security log that never happened.
    Measurement { secure_boot: false, kernel_signature_verified: false }
}
