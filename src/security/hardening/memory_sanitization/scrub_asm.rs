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

//! The bridge to `scrub.S`. The routine lives whole in the assembly file so
//! the artifact a reviewer audits is the artifact that runs; this module only
//! assembles it into the image and declares its signature.

use core::arch::global_asm;

global_asm!(include_str!("scrub.S"));

extern "C" {
    /// Zero `len` bytes at `ptr` with architecturally ordered stores.
    ///
    /// # Safety
    /// `ptr` must be valid for writes of `len` bytes. The routine writes
    /// exactly that range and touches no other memory.
    pub fn nonos_scrub_bytes(ptr: *mut u8, len: usize);
}

#[cfg(test)]
mod tests {
    use super::nonos_scrub_bytes;

    /// Every length and alignment offset through a few words, then a
    /// megabyte: the scrub zeroes exactly the range and the sentinel bytes
    /// on both sides survive.
    #[test]
    fn scrubs_exactly_the_range() {
        for off in 0..8usize {
            for len in 0..64usize {
                let mut buf = [0xA5u8; 96];
                unsafe { nonos_scrub_bytes(buf.as_mut_ptr().add(8 + off), len) };
                for (i, b) in buf.iter().enumerate() {
                    let inside = i >= 8 + off && i < 8 + off + len;
                    assert_eq!(*b == 0, inside, "offset {off} len {len} byte {i}");
                }
            }
        }
        let mut big = alloc::vec![0x5Au8; 1 << 20];
        unsafe { nonos_scrub_bytes(big.as_mut_ptr(), big.len()) };
        assert!(big.iter().all(|&b| b == 0), "the megabyte scrub left a byte");
    }
}
