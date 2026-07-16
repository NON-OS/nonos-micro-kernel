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

    // VM/paging must be ready before any process creator runs. The
    // process subsystem only initializes its tables after this; the
    // userspace init process itself is created exactly once in
    // `microkernel_main`.
    if let Err(e) = crate::memory::unified::init_unified_vm() {
        fatal("memory: init_unified_vm failed", e);
    }
    crate::sys::bench::mark(b"vm_ready");
    // The framebuffer is MMIO-mapped only now: mapping it needs the paging
    // manager, which init_unified_vm brings up. Doing it in the early
    // memory/framebuffer step failed to map on real GOP framebuffers
    // because the page-table machinery was not ready yet.
    init_arch_framebuffer(handoff);
    // Cache the real BSP LAPIC ID into the arch APIC module before any IOAPIC
    // redirection entry is programmed, so device interrupts route to the CPU
    // that actually exists rather than the default of APIC 0 (correct only on
    // QEMU). This only writes the ID cache; it does not touch the arch APIC
    // mode/base/init state, so the timer and IPI paths are unaffected.
    crate::arch::x86_64::interrupt::apic::cache_bsp_apic_id();
    match crate::arch::init_broker_irq_routing() {
        Ok(_) => boot_log::ok("NONOS", "broker IO-APIC routing ready"),
        Err(_) => crate::sys::serial::println(b"[NONOS] broker IO-APIC init failed"),
    }
    crate::process::init_process_management();
    crate::elf::loader::init_elf_loader();
    crate::crypto::kernel_keys::init();
    crate::sys::bench::mark(b"process_runtime_ready");

    super::start_secondary::start_secondary_cpus();

    boot_log::ok("NONOS", "Core ready");
    crate::sys::bench::mark(b"microkernel_core_ready");
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

// Render what the ACPI DSDT/SSDT enumeration found for the touchpad onto the
// on-screen boot log, as the last kernel line before userspace so it stays
// visible in a photo. On real hardware this prints the touchpad's true I2C
// slave address, interrupt pin and HID descriptor register: the ground truth
// the blind-probe path could only guess at.
fn log_acpi_touchpad_onscreen() {
    let devices = crate::arch::x86_64::acpi::aml::enumerate_i2c_hid();
    let Some(d) = devices.first() else {
        boot_log::stage("ACPI-HID", "no i2c-hid device in ACPI DSDT/SSDT");
        return;
    };
    let mut buf = [0u8; 96];
    let mut n = 0;
    n += put(&mut buf[n..], b"addr=0x");
    n += put_hex(&mut buf[n..], d.slave_addr as u64);
    n += put(&mut buf[n..], b" gpio=");
    n += put_dec(&mut buf[n..], d.gpio_pin as u64);
    n += put(&mut buf[n..], b" reg=0x");
    n += put_hex(&mut buf[n..], d.hid_desc_reg as u64);
    n += put(&mut buf[n..], b" hid=");
    for &c in d.hid.iter() {
        if c == 0 {
            break;
        }
        if n < buf.len() {
            buf[n] = c;
            n += 1;
        }
    }
    n += put(&mut buf[n..], b" ctrl=");
    for &c in d.controller.iter() {
        if c == 0 {
            break;
        }
        if n < buf.len() {
            buf[n] = c;
            n += 1;
        }
    }
    let msg = core::str::from_utf8(&buf[..n]).unwrap_or("acpi-hid: format error");
    boot_log::stage("ACPI-HID", msg);
}

fn put(out: &mut [u8], s: &[u8]) -> usize {
    let k = s.len().min(out.len());
    out[..k].copy_from_slice(&s[..k]);
    k
}

fn put_dec(out: &mut [u8], mut v: u64) -> usize {
    let mut tmp = [0u8; 20];
    let mut k = 0;
    loop {
        tmp[k] = b'0' + (v % 10) as u8;
        v /= 10;
        k += 1;
        if v == 0 {
            break;
        }
    }
    for i in 0..k.min(out.len()) {
        out[i] = tmp[k - 1 - i];
    }
    k.min(out.len())
}

fn put_hex(out: &mut [u8], mut v: u64) -> usize {
    let mut tmp = [0u8; 16];
    let mut k = 0;
    loop {
        let d = (v & 0xF) as u8;
        tmp[k] = if d < 10 { b'0' + d } else { b'a' + d - 10 };
        v >>= 4;
        k += 1;
        if v == 0 {
            break;
        }
    }
    for i in 0..k.min(out.len()) {
        out[i] = tmp[k - 1 - i];
    }
    k.min(out.len())
}

pub fn microkernel_main() -> ! {
    crate::sys::bench::mark(b"microkernel_main_start");
    log_acpi_touchpad_onscreen();
    crate::hardware::broker::device_census();
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
    crate::sys::bench::mark(b"init_process_created");
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
    crate::sys::bench::mark(b"init_enter_userspace");
    crate::userspace::run_init()
}
