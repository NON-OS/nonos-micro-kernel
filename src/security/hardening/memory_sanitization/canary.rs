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

use super::state::STACK_CANARY;
use core::sync::atomic::Ordering;

pub fn init_stack_canary() {
    let random_canary = crate::crypto::secure_random_u64();
    STACK_CANARY.store(random_canary, Ordering::SeqCst);
}

#[inline(always)]
pub fn get_stack_canary() -> u64 {
    STACK_CANARY.load(Ordering::Relaxed)
}

#[inline(always)]
pub fn verify_stack_canary(canary: u64) -> bool {
    let expected = STACK_CANARY.load(Ordering::Relaxed);
    let diff = canary ^ expected;
    diff == 0
}

#[inline(never)]
pub fn stack_canary_failed() -> ! {
    crate::log::error!("[SECURITY] STACK CANARY CORRUPTION DETECTED!");
    crate::log::error!("[SECURITY] Possible stack buffer overflow attack!");

    crate::security::audit::log_security_violation(
        alloc::string::String::from("Stack canary corruption detected"),
        crate::security::audit::AuditSeverity::Emergency,
    );

    // The stack this frame stands on is already corrupt, so there is no safe
    // way back. Mask interrupts and stop the CPU here.
    crate::arch::disable_interrupts();
    crate::arch::halt_loop()
}
