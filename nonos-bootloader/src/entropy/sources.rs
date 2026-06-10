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

pub use super::rdrand::{rdrand64, rdseed64};

#[inline(always)]
pub fn rdtsc_serialized() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("lfence", options(nostack, nomem));

        let mut hi: u32;
        let mut lo: u32;
        core::arch::asm!(
            "rdtsc",
            out("eax") lo,
            out("edx") hi,
            options(nomem, nostack, preserves_flags)
        );

        core::arch::asm!("lfence", options(nostack, nomem));

        ((hi as u64) << 32) | (lo as u64)
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

pub fn collect_hw_rng_bytes(buf: &mut [u8; 64], iterations: usize) {
    let mut off = 0usize;

    for i in 0..iterations {
        if let Some(x) = rdseed64() {
            let b = x.to_le_bytes();
            for j in 0..8 {
                buf[(off + j) % 64] ^= b[j];
            }
            off += 8;
        } else {
            for _ in 0..10 {
                if let Some(x) = rdrand64() {
                    let b = x.to_le_bytes();
                    for j in 0..8 {
                        buf[(off + j) % 64] ^= b[j];
                    }
                    off += 8;
                    break;
                }
                for _ in 0..50 {
                    core::hint::spin_loop();
                }
            }
        }
        let t1 = rdtsc_serialized();
        for _ in 0..((i % 16) + 1) {
            core::hint::spin_loop();
        }
        let t2 = rdtsc_serialized();
        let jb = t2.wrapping_sub(t1).to_le_bytes();
        for j in 0..8 {
            buf[(off + j + 32) % 64] ^= jb[j];
        }
    }

    for i in 0..64 {
        let t = rdtsc_serialized();
        for _ in 0..((i % 8) + 1) {
            core::hint::spin_loop();
        }
        buf[i] ^= (t >> ((i % 8) * 8)) as u8;
    }
}
