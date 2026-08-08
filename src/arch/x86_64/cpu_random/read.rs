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

//! RDRAND and RDSEED.
//!
//! Both report success in CF, so neither asm block may claim `preserves_flags`.
//! The retry budgets follow Intel's guidance: RDRAND is served from a buffered
//! DRBG and ten attempts is generous, while RDSEED waits on the physical
//! conditioner and is expected to fail far more often under contention.

const RDRAND_ATTEMPTS: u32 = 10;
const RDSEED_ATTEMPTS: u32 = 100;

/// Draw from the conditioned DRBG.
///
/// # Safety
///
/// The CPU must implement RDRAND; executing it otherwise raises #UD. Callers
/// go through `arch::cpu_random`, which checks first.
pub unsafe fn rdrand_u64() -> Option<u64> {
    for _ in 0..RDRAND_ATTEMPTS {
        let value: u64;
        let ok: u8;
        // SAFETY: the caller guarantees RDRAND is present. The instruction
        // writes only the two output operands and CF, touches no memory, and
        // needs no stack.
        unsafe {
            core::arch::asm!(
                "rdrand {value}",
                "setc {ok}",
                value = out(reg) value,
                ok = out(reg_byte) ok,
                options(nomem, nostack),
            );
        }
        if ok != 0 {
            return Some(value);
        }
        core::hint::spin_loop();
    }
    None
}

/// Draw from the reseeded entropy conditioner.
///
/// # Safety
///
/// The CPU must implement RDSEED; executing it otherwise raises #UD. Callers
/// go through `arch::cpu_random`, which checks first.
pub unsafe fn rdseed_u64() -> Option<u64> {
    for _ in 0..RDSEED_ATTEMPTS {
        let value: u64;
        let ok: u8;
        // SAFETY: the caller guarantees RDSEED is present. Same operand and
        // memory profile as RDRAND above.
        unsafe {
            core::arch::asm!(
                "rdseed {value}",
                "setc {ok}",
                value = out(reg) value,
                ok = out(reg_byte) ok,
                options(nomem, nostack),
            );
        }
        if ok != 0 {
            return Some(value);
        }
        core::hint::spin_loop();
    }
    None
}
