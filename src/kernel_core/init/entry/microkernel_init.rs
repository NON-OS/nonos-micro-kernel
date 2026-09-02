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

//! The boot sequence, as an ordered list of stages. Each stage's file carries
//! the argument for why it sits where it does; the ordering constraints are
//! real and none of these lines move without reading them.

use super::init_arch_firmware::init_arch_firmware;
use super::init_arch_framebuffer::init_arch_framebuffer;
use super::init_arch_memory_and_framebuffer::init_arch_memory_and_framebuffer;
use super::init_core_services::init_core_services;
use super::init_runtime::{init_device_routing, init_process_runtime};
use super::init_vm_and_protection::init_vm_and_protection;
use crate::boot::handoff::KernelHandoff;
use crate::sys::boot_log;

pub fn microkernel_init(handoff: &KernelHandoff) {
    crate::sys::bench::mark(b"microkernel_init_start");
    init_arch_memory_and_framebuffer(handoff);
    let cursor_y = handoff.framebuffer.map(|fb| fb.cursor_y).unwrap_or(0);
    boot_log::init_after_fb(cursor_y);
    boot_log::ok("NONOS", "Microkernel init");

    init_arch_firmware(handoff);
    init_core_services(handoff);
    init_vm_and_protection();

    // Runs here rather than earlier because seeding the broker walks PCI
    // config space, which needs the paging manager to hand out a register
    // window. On x86_64 the boot path has already done this and the call
    // returns without repeating it.
    crate::kernel_core::init::init_platform_baseline();
    // Measured here because the counter frequency and the IPC secret are both
    // up by now, and nothing else is competing for the machine yet, which is
    // the only point in a boot where a quantile means anything.
    #[cfg(feature = "nonos-bench-micro")]
    crate::sys::microbench::run();
    // The framebuffer is MMIO-mapped only now: mapping it needs the paging
    // manager, which init_unified_vm brings up. Doing it in the early
    // memory/framebuffer step failed to map on real GOP framebuffers because
    // the page-table machinery was not ready yet.
    init_arch_framebuffer(handoff);

    init_device_routing();
    init_process_runtime();

    // Run here rather than earlier: the crypto the suite times needs its keys
    // up, and the counter needs the clock anchored. Nothing after this point
    // depends on the results, so a slow run delays the boot and breaks nothing.
    #[cfg(feature = "nonos-bench-micro")]
    crate::sys::bench::suite::run_all();

    super::super::start_secondary::start_secondary_cpus();

    boot_log::ok("NONOS", "Core ready");
    crate::sys::bench::mark(b"microkernel_core_ready");
}
