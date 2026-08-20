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

use crate::smp::state::{
    BSP_APIC_ID, BSP_INITIALIZING, CPU_COUNT, CPU_DESCRIPTORS, SMP_INITIALIZED,
};
use crate::smp::CpuState;
use crate::smp::{percpu, topology};
use core::sync::atomic::Ordering;

pub fn init_bsp() -> Result<(), &'static str> {
    if !claim_bsp_init() {
        wait_until_ready();
        return Ok(());
    }

    let bsp_apic = crate::arch::interrupt_controller::local_id();
    BSP_APIC_ID.store(bsp_apic, Ordering::Release);
    configure_bsp_descriptor(bsp_apic);

    let cpu_count = topology::detect_cpus();
    CPU_COUNT.store(cpu_count, Ordering::Release);
    percpu::init_bsp();
    // Bind the IPI vectors now so cross-CPU TLB shootdown, panic, stop, and
    // call-function delivery work the moment APs come online. Harmless on a
    // single-CPU boot: nothing sends IPIs until APs start.
    super::super::ipi_dispatch::register_ipi_handlers();

    crate::log_info!("[SMP] BSP initialized: APIC ID={}, {} CPUs detected", bsp_apic, cpu_count);

    SMP_INITIALIZED.store(true, Ordering::Release);
    Ok(())
}

fn claim_bsp_init() -> bool {
    BSP_INITIALIZING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok()
}

fn wait_until_ready() {
    while !SMP_INITIALIZED.load(Ordering::Acquire) {
        core::hint::spin_loop();
    }
}

fn configure_bsp_descriptor(apic_id: u32) {
    let bsp = &CPU_DESCRIPTORS[0];
    // numa_node is already zero from the const initialiser, so nothing here
    // needs to write it.
    bsp.set_identity(0, apic_id, 0);
    bsp.set_state(CpuState::Online);
}
