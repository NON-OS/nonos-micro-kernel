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

use super::fatal::fatal;
use super::init_arch_firmware::init_arch_firmware;
use super::init_arch_framebuffer::init_arch_framebuffer;
use super::init_arch_memory_and_framebuffer::init_arch_memory_and_framebuffer;
use crate::boot::handoff::KernelHandoff;
use crate::sys::{boot_log, clock};

pub fn microkernel_init(handoff: &KernelHandoff) {
    crate::sys::bench::mark(b"microkernel_init_start");
    init_arch_memory_and_framebuffer(handoff);
    let cursor_y = handoff.framebuffer.map(|fb| fb.cursor_y).unwrap_or(0);
    boot_log::init_after_fb(cursor_y);
    boot_log::ok("NONOS", "Microkernel init");

    init_arch_firmware(handoff);
    crate::sys::policy::hostname_init();
    if let Err(_) = crate::crypto::util::rng::init_rng() {
        fatal("crypto: init_rng failed", "entropy unavailable");
    }
    if let Err(e) = crate::ipc::nonos_channel::init_ipc_secret() {
        fatal("ipc: init_ipc_secret failed", e);
    }
    if let Err(e) = crate::smp::init_bsp() {
        fatal("smp: init_bsp failed", e);
    }
    crate::sched::init();
    clock::init(handoff.timing.fixed_freq_hz.unwrap_or(0), handoff.timing.unix_epoch_ms);
    // The wall clock and the uptime counter hold their own copies of the
    // counter frequency. Seeding only the first leaves uptime reporting zero
    // forever, and every wait on elapsed time then never finishes. The x86_64
    // binary seeds the second from `init_core_systems`, which no other entry
    // path runs, so it belongs here where every arch passes through.
    crate::sys::timer::tsc::init(
        handoff.timing.fixed_freq_hz.unwrap_or(0),
        handoff.timing.unix_epoch_ms,
    );

    // VM/paging must be ready before any process creator runs. The
    // process subsystem only initializes its tables after this; the
    // userspace init process itself is created exactly once in
    // `microkernel_main`.
    if let Err(e) = crate::memory::unified::init_unified_vm() {
        fatal("memory: init_unified_vm failed", e);
    }
    crate::sys::bench::mark(b"vm_ready");
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
    // memory/framebuffer step failed to map on real GOP framebuffers
    // because the page-table machinery was not ready yet.
    init_arch_framebuffer(handoff);
    // Latch the boot CPU's controller id before any redirection entry is
    // programmed, so device interrupts route to the CPU that actually exists
    // rather than to id 0, which is only correct under QEMU. This writes the
    // id cache and nothing else: controller mode, base and init state are
    // untouched, so the timer and IPI paths are unaffected.
    crate::arch::interrupt_controller::cache_boot_cpu_id();
    // Named for the job, not the part: x86_64 does this with an IO-APIC and
    // aarch64 with the GIC distributor, which is already up by the time this
    // runs.
    match crate::arch::init_broker_irq_routing() {
        Ok(_) => boot_log::ok("NONOS", "device interrupt routing ready"),
        Err(_) => crate::sys::serial::println(b"[NONOS] device interrupt routing failed"),
    }
    crate::process::init_process_management();
    crate::elf::loader::init_elf_loader();
    crate::crypto::kernel_keys::init();
    crate::sys::bench::mark(b"process_runtime_ready");

    // Run here rather than earlier: the crypto the suite times needs its keys
    // up, and the counter needs the clock anchored. Nothing after this point
    // depends on the results, so a slow run delays the boot and breaks nothing.
    #[cfg(feature = "nonos-bench-micro")]
    crate::sys::bench::suite::run_all();

    super::super::start_secondary::start_secondary_cpus();

    boot_log::ok("NONOS", "Core ready");
    crate::sys::bench::mark(b"microkernel_core_ready");
}
