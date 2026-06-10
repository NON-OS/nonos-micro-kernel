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

use crate::entropy::rdrand64;
use crate::security::{audit, init_canaries, AuditEvent};

pub fn init_security_primitives() {
    let entropy = match rdrand64() {
        Some(v) => v,
        None => {
            let (lo, hi): (u32, u32);
            unsafe {
                core::arch::asm!("rdtsc", out("eax") lo, out("edx") hi, options(nomem, nostack, preserves_flags));
            }
            (((hi as u64) << 32) | lo as u64).rotate_left(17) ^ 0x9E37_79B9_7F4A_7C15
        }
    };
    init_canaries(entropy);
    audit(AuditEvent::BootStart, 0, b"security init");
}
