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

#![no_std]
#![no_main]

extern crate alloc;
extern crate nonos_kernel;

#[cfg(target_arch = "x86_64")]
use core::sync::atomic::{AtomicU64, Ordering};

mod manifest_embed {
    include!(concat!(env!("OUT_DIR"), "/manifest_data.rs"));
}
pub use manifest_embed::*;

// The entry below is the one a UEFI bootloader jumps to, and it takes the
// handoff structure that bootloader builds. aarch64 arrives somewhere else
// entirely: firmware enters `_start` in arch/aarch64/asm/start.S, which drops to
// EL1 and calls the `kernel_entry` in arch::aarch64::boot::entry with a device
// tree pointer instead. Two entries, one per boot protocol, and only the one
// belonging to the target is compiled so the symbol is defined once.
#[cfg(target_arch = "x86_64")]
use nonos_kernel::boot::handoff::init_handoff;
#[cfg(target_arch = "x86_64")]
use nonos_kernel::boot::main::init_core_systems;
#[cfg(target_arch = "x86_64")]
use nonos_kernel::entry::{fallback, security};
#[cfg(target_arch = "x86_64")]
use nonos_kernel::sys::serial;

// `_start` lives in arch/x86_64/asm/start.S; calls in with rdi=handoff_ptr.
#[cfg(target_arch = "x86_64")]
static HANDOFF_PTR: AtomicU64 = AtomicU64::new(0);

#[cfg(target_arch = "x86_64")]
#[no_mangle]
extern "C" fn kernel_entry(handoff_ptr: u64) -> ! {
    // First instruction path: paint the entry breadcrumb from the raw,
    // unvalidated handoff before anything that could hang or halt runs.
    // On a machine with no serial console this orange bar is the only
    // proof the jump from the bootloader ever landed.
    if handoff_ptr != 0 {
        let raw = unsafe { &*(handoff_ptr as *const nonos_kernel::boot::handoff::BootHandoffV1) };
        nonos_kernel::boot::entry_marker::paint(raw, 0, 0xFFFF_8000);
    }
    unsafe {
        core::arch::asm!(
            "mov dx, 0x3F8", "mov al, 'R'", "out dx, al",
            "mov al, 0x0A", "out dx, al",
            out("dx") _, out("al") _,
        );
    }
    serial::init();
    HANDOFF_PTR.store(handoff_ptr, Ordering::SeqCst);
    if handoff_ptr == 0 {
        serial::println(b"[NONOS] CRITICAL: No handoff!");
        fallback::vga_fallback();
    }
    let handoff = match unsafe { init_handoff(handoff_ptr) } {
        Ok(h) => {
            serial::println(b"[NONOS] Handoff OK");
            h
        }
        Err(err) => {
            serial::println(b"[NONOS] Handoff FAIL");
            serial::print(b"[NONOS] Handoff ERR: ");
            serial::print_str(err.as_str());
            serial::println(b"");
            let raw =
                unsafe { &*(handoff_ptr as *const nonos_kernel::boot::handoff::BootHandoffV1) };
            nonos_kernel::boot::entry_marker::paint(raw, 0, 0xFFFF_0000);
            fallback::vga_fallback();
        }
    };
    nonos_kernel::boot::entry_marker::paint(handoff, 0, 0xFF00_FFFF);
    init_core_systems();
    security::log_security_status(handoff);
    nonos_kernel::boot::entry_marker::paint(handoff, 1, 0xFFFF_D000);
    boot_microkernel(handoff)
}

#[cfg(target_arch = "x86_64")]
fn boot_microkernel(handoff: &nonos_kernel::boot::handoff::BootHandoffV1) -> ! {
    if handoff.fb.ptr == 0 {
        serial::println(b"[NONOS] No boot framebuffer; continuing with capsule graphics");
    }
    serial::println(b"[NONOS] Microkernel boot");
    let kernel_handoff = nonos_kernel::boot::handoff::KernelHandoff::from_x86_64(handoff);
    nonos_kernel::kernel_core::microkernel_init(&kernel_handoff);

    nonos_kernel::kernel_core::microkernel_main()
}
