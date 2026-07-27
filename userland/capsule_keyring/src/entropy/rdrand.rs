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

// RDRAND is an unprivileged instruction, so a ring-3 capsule may execute it
// directly. It is the on-die NIST SP800-90 DRBG and stays available even when
// the kernel entropy pool is momentarily unreachable (as under a hypervisor
// that starves the entropy capsule), so it is the keyring's own hardware source
// of last resort for key material. Never a fallback to a clock or counter: a
// draw either yields real hardware entropy or reports failure.

#[cfg(target_arch = "x86_64")]
fn rdrand64() -> Option<u64> {
    // The Intel guidance is to retry a bounded number of times; ten is the
    // documented ceiling before treating the DRBG as unavailable.
    for _ in 0..10 {
        let val: u64;
        let ok: u8;
        unsafe {
            core::arch::asm!(
                "rdrand {v}",
                "setc {c}",
                v = out(reg) val,
                c = out(reg_byte) ok,
                options(nostack, nomem),
            );
        }
        if ok != 0 && val != 0 {
            return Some(val);
        }
    }
    None
}

#[cfg(target_arch = "x86_64")]
fn rdseed64() -> Option<u64> {
    for _ in 0..10 {
        let val: u64;
        let ok: u8;
        unsafe {
            core::arch::asm!(
                "rdseed {v}",
                "setc {c}",
                v = out(reg) val,
                c = out(reg_byte) ok,
                options(nostack, nomem),
            );
        }
        if ok != 0 {
            return Some(val);
        }
    }
    None
}

#[cfg(not(target_arch = "x86_64"))]
fn rdrand64() -> Option<u64> {
    None
}

#[cfg(not(target_arch = "x86_64"))]
fn rdseed64() -> Option<u64> {
    None
}

/// Fill `buf` with hardware entropy drawn eight bytes at a time. Each word
/// prefers a fresh RDSEED (full-entropy seed) and falls back to RDRAND. Returns
/// false, and leaves nothing partial to rely on, if the hardware DRBG cannot be
/// read; the caller wipes and reports failure rather than proceed.
pub fn fill_hardware(buf: &mut [u8]) -> bool {
    let mut off = 0;
    while off < buf.len() {
        let word = match rdseed64().or_else(rdrand64) {
            Some(w) => w,
            None => return false,
        };
        let bytes = word.to_le_bytes();
        let take = core::cmp::min(8, buf.len() - off);
        buf[off..off + take].copy_from_slice(&bytes[..take]);
        off += take;
    }
    true
}
