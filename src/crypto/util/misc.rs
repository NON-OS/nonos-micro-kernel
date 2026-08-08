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

extern crate alloc;
use super::entropy::get_entropy;

pub fn secure_random_u64() -> u64 {
    let entropy = get_entropy(8);
    if entropy.len() >= 8 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&entropy[..8]);
        u64::from_le_bytes(bytes)
    } else {
        crate::arch::read_time_counter()
    }
}

pub fn secure_random_u32() -> u32 {
    (secure_random_u64() >> 32) as u32
}

pub fn secure_random_bytes(buffer: &mut [u8]) {
    let entropy = get_entropy(buffer.len());
    buffer.copy_from_slice(&entropy[..buffer.len()]);
}

/// # Safety
/// Constant-time byte slice comparison. Compares all bytes regardless of
/// length mismatch. Returns true only if lengths match AND all bytes match.
/// Timing is independent of where differences occur.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    let len_a = a.len();
    let len_b = b.len();

    let mut diff = (len_a ^ len_b) as u8;

    let min_len = if len_a < len_b { len_a } else { len_b };
    let max_len = if len_a > len_b { len_a } else { len_b };

    for i in 0..min_len {
        diff |= a[i] ^ b[i];
    }

    for i in min_len..max_len {
        let byte_a = if i < len_a { a[i] } else { 0 };
        let byte_b = if i < len_b { b[i] } else { 0 };
        diff |= byte_a ^ byte_b ^ 0xFF;
    }

    diff == 0
}

pub fn secure_zero(buffer: &mut [u8]) {
    for byte in buffer.iter_mut() {
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
