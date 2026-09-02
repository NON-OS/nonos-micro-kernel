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

use crate::smp::constants::MAX_CPUS;
use crate::smp::state::{BSP_APIC_ID, CPUS_ONLINE, CPU_COUNT, SMP_INITIALIZED};
use crate::smp::topology;
use core::sync::atomic::Ordering;

pub(super) fn start_aps() -> Result<usize, &'static str> {
    ensure_smp_ready()?;

    let cpu_count = CPU_COUNT.load(Ordering::Acquire);
    if cpu_count <= 1 {
        crate::log_info!("[SMP] Single CPU system, no APs to start");
        return Ok(0);
    }

    let boot = super::boot_inputs::prepare()?;

    // The trampoline loads a CR3 whose low half is cleared, so it needs the
    // trampoline region identity-mapped to survive the switch to long mode.
    // Removed once the APs are up, before any userspace runs.
    super::ap_identity::install()?;

    let mut started = 0;
    // Handed out once each and never reused, unlike `started`, which does not
    // advance when an AP fails. An AP that missed its deadline was not stopped
    // and may still be walking the trampoline; reusing its number would point
    // two live CPUs at one descriptor and one per-CPU block.
    let mut next_cpu_id = 1;
    let bsp_apic = BSP_APIC_ID.load(Ordering::Acquire);
    let ap_list = topology::get_ap_list();

    for &apic_id in &ap_list {
        if apic_id == bsp_apic {
            continue;
        }

        let cpu_id = next_cpu_id;
        if cpu_id >= MAX_CPUS {
            crate::log_warn!("[SMP] CPU limit {} reached, rest left offline", MAX_CPUS);
            break;
        }
        next_cpu_id += 1;

        if super::ap_unit::start(cpu_id, apic_id, &boot).unwrap_or(false) {
            started += 1;
        }
    }
    super::ap_identity::remove();

    CPUS_ONLINE.store(started + 1, Ordering::Release);
    crate::log_info!("[SMP] {} APs started, {} total CPUs online", started, started + 1);
    Ok(started)
}

fn ensure_smp_ready() -> Result<(), &'static str> {
    if SMP_INITIALIZED.load(Ordering::Acquire) {
        Ok(())
    } else {
        Err("SMP not initialized")
    }
}
