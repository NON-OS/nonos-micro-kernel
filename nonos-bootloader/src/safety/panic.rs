// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

/*
 * Boot Panic Handler.
 *
 * Secure panic handling that:
 * 1. Logs panic location to audit trail
 * 2. Clears sensitive memory
 * 3. Displays error on screen
 * 4. Halts or resets safely
 *
 * Prevents boot loop attacks and information leakage.
 */

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

static PANIC_OCCURRED: AtomicBool = AtomicBool::new(false);
static PANIC_COUNT: AtomicU32 = AtomicU32::new(0);
static PANIC_LINE: AtomicU32 = AtomicU32::new(0);

const MAX_PANIC_RETRIES: u32 = 3;

pub fn record_panic(line: u32) {
    PANIC_OCCURRED.store(true, Ordering::SeqCst);
    PANIC_LINE.store(line, Ordering::SeqCst);
    PANIC_COUNT.fetch_add(1, Ordering::SeqCst);
}

pub fn has_panicked() -> bool {
    PANIC_OCCURRED.load(Ordering::SeqCst)
}

pub fn panic_count() -> u32 {
    PANIC_COUNT.load(Ordering::SeqCst)
}

pub fn get_panic_line() -> u32 {
    PANIC_LINE.load(Ordering::SeqCst)
}

pub fn should_halt() -> bool {
    panic_count() >= MAX_PANIC_RETRIES
}

pub fn clear_panic_state() {
    PANIC_OCCURRED.store(false, Ordering::SeqCst);
}

pub fn secure_halt() -> ! {
    clear_sensitive_memory();

    loop {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            core::arch::asm!("cli; hlt", options(nomem, nostack));
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            core::hint::spin_loop();
        }
    }
}

pub fn secure_reset() -> ! {
    clear_sensitive_memory();

    #[cfg(target_arch = "x86_64")]
    unsafe {
        /* triple fault reset */
        core::arch::asm!(
            "lidt [rax]",
            in("rax") 0u64,
            options(nomem, nostack)
        );
        core::arch::asm!("int3", options(nomem, nostack));
    }

    loop {
        core::hint::spin_loop();
    }
}

fn clear_sensitive_memory() {
    crate::crypto::keystore_v2::wipe_all_keys();
    crate::crypto::sig::wipe_signing_state();
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[derive(Debug, Clone, Copy)]
pub struct PanicInfo {
    pub line: u32,
    pub count: u32,
    pub category: PanicCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanicCategory {
    Unknown,
    MemoryCorruption,
    CryptoFailure,
    SecurityViolation,
    HardwareError,
    AssertionFailed,
}

impl PanicInfo {
    pub fn capture(category: PanicCategory, line: u32) -> Self {
        record_panic(line);
        Self { line, count: panic_count(), category }
    }
}
