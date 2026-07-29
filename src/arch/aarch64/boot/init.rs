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

use super::info::BootInfo;
use crate::arch::aarch64::{cpu, exceptions, gic, mmu, security, timer, uart};

pub fn init(boot_info: &BootInfo) {
    uart::init_uart(boot_info.uart_base);
    cpu::init_cpu();
    // Latch the CPU list before anything asks which CPU it is running on. The
    // GIC does, one call below, and its answer decides which redistributor
    // this core talks to.
    //
    // SAFETY: this is the boot CPU, this runs once, and no secondary has been
    // released yet, so nothing else can be touching the roster.
    unsafe {
        super::multicore::roster::populate(&boot_info.cpu_affinities);
    }
    // Port numbers are offsets into the bridge's I/O window on this arch, so
    // the window has to be known before any driver reaches for a port. ECAM is
    // the only way to config space here, and enumeration needs it.
    crate::arch::port_io::set_io_window(
        boot_info.pci_io_cpu_base,
        boot_info.pci_io_port_base,
        boot_info.pci_io_size,
    );
    crate::drivers::pci::set_ecam_window(boot_info.pci_ecam_base, boot_info.pci_ecam_size);
    crate::arch::aarch64::rtc::set_base(boot_info.rtc_base);
    exceptions::install_vbar_el1();
    if security::init_all().is_err() {
        cpu::halt();
    }
    mmu::init_mmu(boot_info);
    if boot_info.gic_unsupported {
        cpu::halt();
    }
    gic::init_gic(boot_info.gic_dist_base, boot_info.gic_redist_base);
    timer::init_timer();
    timer::configure_preemption_intid(boot_info.timer_phys_intid);
    if timer::install_on_cpu().is_err() {
        cpu::halt();
    }
    if super::multicore::roster::len() > 1 {
        super::multicore::start_secondary_cpus(boot_info);
    }
}
