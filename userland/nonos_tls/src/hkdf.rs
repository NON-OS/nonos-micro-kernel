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

use alloc::vec::Vec;

pub fn extract(salt: &[u8], ikm: &[u8]) -> Option<[u8; 32]> {
    let mut out = [0u8; 32];
    let n = nonos_libc::crypto_hmac_sha256(
        salt.as_ptr(),
        salt.len(),
        ikm.as_ptr(),
        ikm.len(),
        out.as_mut_ptr(),
    );
    if n == 32 {
        Some(out)
    } else {
        None
    }
}

pub fn expand(prk: &[u8; 32], info: &[u8], out: &mut [u8]) -> bool {
    let mut prev: Vec<u8> = Vec::new();
    let mut done = 0usize;
    let mut counter = 1u8;
    while done < out.len() {
        let mut data = Vec::with_capacity(prev.len() + info.len() + 1);
        data.extend_from_slice(&prev);
        data.extend_from_slice(info);
        data.push(counter);
        let Some(block) = extract(prk, &data) else { return false };
        let n = core::cmp::min(block.len(), out.len() - done);
        out[done..done + n].copy_from_slice(&block[..n]);
        prev.clear();
        prev.extend_from_slice(&block);
        done += n;
        counter = counter.wrapping_add(1);
    }
    true
}
