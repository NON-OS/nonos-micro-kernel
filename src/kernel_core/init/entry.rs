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
#[cfg(target_arch = "x86_64")]
use super::memory::init_memory;
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

    crate::sys::serial::println(b"[INIT-TRACE] before firmware");
    init_arch_firmware(handoff);
    crate::sys::serial::println(b"[INIT-TRACE] after firmware");

    crate::sys::serial::println(b"[INIT-TRACE] before settings");
    crate::sys::settings::init();
    crate::sys::serial::println(b"[INIT-TRACE] after settings");

    crate::sys::serial::println(b"[INIT-TRACE] before hostname");
    crate::sys::settings::init_hostname();
    crate::sys::serial::println(b"[INIT-TRACE] after hostname");

    crate::sys::serial::println(b"[INIT-TRACE] before rng");
    if let Err(_) = crate::crypto::util::rng::init_rng() {
        fatal("crypto: init_rng failed", "entropy unavailable");
    }
    crate::sys::serial::println(b"[INIT-TRACE] after rng");

    crate::sys::serial::println(b"[INIT-TRACE] before ipc-secret");
    if let Err(e) = crate::ipc::nonos_channel::init_ipc_secret() {
        fatal("ipc: init_ipc_secret failed", e);
    }
    crate::sys::serial::println(b"[INIT-TRACE] after ipc-secret");

    crate::sys::serial::println(b"[INIT-TRACE] before percpu-bsp");
    if let Err(e) = crate::smp::init_bsp() {
        fatal("smp: init_bsp failed", e);
    }
    crate::sys::serial::println(b"[INIT-TRACE] after percpu-bsp");

    crate::sys::serial::println(b"[INIT-TRACE] before sched");
    crate::sched::init();
    crate::sys::serial::println(b"[INIT-TRACE] after sched");

    crate::sys::serial::println(b"[INIT-TRACE] before clock");
    clock::init(handoff.timing.fixed_freq_hz.unwrap_or(0), handoff.timing.unix_epoch_ms);
    crate::sys::serial::println(b"[INIT-TRACE] after clock");

    // VM/paging must be ready before any process creator runs. The
    // process subsystem only initializes its tables after this; the
    // userspace init process itself is created exactly once in
    // `microkernel_main`.
    crate::sys::serial::println(b"[INIT-TRACE] before unified-vm");
    if let Err(e) = crate::memory::unified::init_unified_vm() {
        fatal("memory: init_unified_vm failed", e);
    }
    crate::sys::serial::println(b"[INIT-TRACE] after unified-vm");

    crate::sys::serial::println(b"[INIT-TRACE] before process-management");
    crate::process::init_process_management();
    crate::sys::serial::println(b"[INIT-TRACE] after process-management");

    crate::sys::serial::println(b"[INIT-TRACE] before elf-loader");
    crate::elf::loader::init_elf_loader();
    crate::sys::serial::println(b"[INIT-TRACE] after elf-loader");

    crate::sys::serial::println(b"[INIT-TRACE] before kernel-keys");
    crate::crypto::kernel_keys::init();
    crate::sys::serial::println(b"[INIT-TRACE] after kernel-keys");

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
            #[cfg(target_arch = "x86_64")]
            init_memory(v1);
            init_framebuffer(v1);
            super::framebuffer::draw_desktop();
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
    CURRENT_PID.store(init_pid, Ordering::SeqCst);
    boot_log::ok("UKERNEL", "Entering userspace");
    crate::userspace::run_init()
}
