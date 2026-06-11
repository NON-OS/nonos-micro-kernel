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

use super::framebuffer::init_framebuffer;
use crate::boot::handoff::{ArchSpecificHandoff, KernelHandoff};
use crate::memory::paging::manager::api::create_address_space;
use crate::process::core::{create_process, Priority, ProcessState, CURRENT_PID};
use crate::sys::{boot_log, clock};
use core::sync::atomic::Ordering;

pub fn microkernel_init(handoff: &KernelHandoff) {
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

    // VM/paging must be ready before any process creator runs. The
    // process subsystem only initializes its tables after this; the
    // userspace init process itself is created exactly once in
    // `microkernel_main`.
    if let Err(e) = crate::memory::unified::init_unified_vm() {
        fatal("memory: init_unified_vm failed", e);
    }
    // The framebuffer is MMIO-mapped only now: mapping it needs the paging
    // manager, which init_unified_vm brings up. Doing it in the early
    // memory/framebuffer step failed to map on real GOP framebuffers
    // because the page-table machinery was not ready yet.
    init_arch_framebuffer(handoff);
    match crate::arch::init_broker_irq_routing() {
        Ok(_) => boot_log::ok("NONOS", "broker IO-APIC routing ready"),
        Err(_) => crate::sys::serial::println(b"[NONOS] broker IO-APIC init failed"),
    }
    crate::process::init_process_management();
    crate::elf::loader::init_elf_loader();
    crate::crypto::kernel_keys::init();

    #[cfg(feature = "nonos-selftest")]
    {
        crate::sys::serial::println(b"[SELFTEST] running boot::tests::selftest::run_all");
        let report = crate::boot::tests::selftest::run_all();
        if report.all_passed() {
            crate::sys::serial::println(b"[SELFTEST] PASS handoff_security");
        } else {
            crate::sys::serial::println(b"[SELFTEST] FAIL one or more groups failed");
            if !report.handoff_security {
                crate::sys::serial::println(b"[SELFTEST]   FAIL handoff_security");
            }
        }
    }

    super::start_secondary::start_secondary_cpus();

    boot_log::ok("NONOS", "Core ready");
}

fn fatal(stage: &str, detail: &str) -> ! {
    boot_log::error(stage);
    crate::sys::serial::print(b"[FATAL] ");
    crate::sys::serial::print_str(stage);
    crate::sys::serial::print(b": ");
    crate::sys::serial::println(detail.as_bytes());
    crate::arch::halt_loop()
}

// EFI memory descriptor walks and UEFI framebuffer init are inherently
// arch-specific. Other arches will add match arms when their boot trees
// land with their own per-arch init helpers.
fn init_arch_memory_and_framebuffer(handoff: &KernelHandoff) {
    match handoff.arch {
        ArchSpecificHandoff::X86_64 { v1 } => {
            crate::arch::init_boot_memory(v1);
        }
    }
}

// Map the handoff framebuffer once the paging manager is initialized.
fn init_arch_framebuffer(handoff: &KernelHandoff) {
    match handoff.arch {
        ArchSpecificHandoff::X86_64 { v1 } => {
            init_framebuffer(v1);
        }
    }
}

// Firmware tables (ACPI/SMBIOS on x86_64; DTB on aarch64/riscv64) are
// arch-specific. Same shape as the memory/framebuffer downcast.
fn init_arch_firmware(handoff: &KernelHandoff) {
    match handoff.arch {
        ArchSpecificHandoff::X86_64 { v1 } => {
            crate::boot::firmware::init(&v1.firmware);
        }
    }
}

pub fn microkernel_main() -> ! {
    boot_log::ok("NONOS", "boot log held; starting userspace");
    let start = clock::uptime_ms();
    let mut guard: u64 = 0;
    while clock::uptime_ms().wrapping_sub(start) < 2500 {
        core::hint::spin_loop();
        guard = guard.wrapping_add(1);
        if guard > 3_000_000_000 {
            break;
        }
    }
    boot_log::ok("UKERNEL", "Creating init");
    let init_pid = match create_process("init", ProcessState::Running, Priority::High) {
        Ok(pid) => pid,
        Err(e) => {
            boot_log::error("Failed to create init process");
            crate::sys::serial::println(b"[FATAL] Init process creation failed");
            crate::sys::serial::println(e.as_bytes());
            crate::arch::halt_loop()
        }
    };
    if let Err(_) = create_address_space(init_pid) {
        boot_log::error("Failed to create init address space");
        crate::sys::serial::println(b"[FATAL] Init address space creation failed");
        crate::arch::halt_loop()
    }
    if crate::kernel_core::process_spawn::allocate_kernel_stack(init_pid).is_err() {
        boot_log::error("Failed to allocate init kernel stack");
        crate::sys::serial::println(b"[FATAL] Init kernel stack allocation failed");
        crate::arch::halt_loop()
    }
    CURRENT_PID.store(init_pid, Ordering::SeqCst);
    boot_log::ok("UKERNEL", "Entering userspace");
    crate::userspace::run_init()
}
