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

use super::sources::{collect_hw_rng_bytes, rdtsc_serialized};

/// C ABI getrandom implementation for host-side libraries that need it.
/// Uses hardware RNG (RDRAND/RDSEED) and TSC jitter for entropy.
#[no_mangle]
pub unsafe extern "C" fn getrandom(buf: *mut u8, len: usize, _flags: u32) -> isize {
    if buf.is_null() || len == 0 {
        return -1;
    }

    let slice = unsafe { core::slice::from_raw_parts_mut(buf, len) };

    // Collect entropy from hardware RNG and TSC
    let mut entropy = [0u8; 64];
    collect_hw_rng_bytes(&mut entropy, 16);

    // Add TSC jitter entropy
    for i in 0..4 {
        let t1 = rdtsc_serialized();
        for _ in 0..100 {
            core::hint::spin_loop();
        }
        let t2 = rdtsc_serialized();
        let delta = t2.wrapping_sub(t1);
        entropy[32 + i * 8..40 + i * 8].copy_from_slice(&delta.to_le_bytes());
    }

    // Expand entropy to fill the buffer
    let mut hasher = blake3::Hasher::new();
    hasher.update(&entropy);
    hasher.finalize_xof().fill(slice);

    len as isize
}
