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

//! Entry bytes and the machine's side of the fold.

use crate::kernel::{DOMAIN, ENTRY_LEN};

/// Capability masks read out of a v0.9.2 boot.
pub const KEYRING: u64 = 0x39;
pub const COMPOSITOR: u64 = 0x7919;
pub const BROWSER: u64 = 0x183D;

/// One entry, encoded as the kernel's `digest_input` encodes it.
pub fn entry(pid: u32, measurement: u8, caps: u64, authority: u8) -> [u8; ENTRY_LEN] {
    let mut e = [0u8; ENTRY_LEN];
    e[..4].copy_from_slice(&pid.to_be_bytes());
    e[4..36].copy_from_slice(&[measurement; 32]);
    e[36..44].copy_from_slice(&caps.to_be_bytes());
    e[44] = authority;
    e
}

/// The kernel's fold: domain, count, then each entry in turn.
pub fn kernel_fold(entries: &[[u8; ENTRY_LEN]]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN);
    hasher.update(&(entries.len() as u32).to_be_bytes());
    for e in entries {
        hasher.update(e);
    }
    *hasher.finalize().as_bytes()
}

pub fn flatten(entries: &[[u8; ENTRY_LEN]]) -> Vec<u8> {
    entries.iter().flatten().copied().collect()
}

/// A registry in miniature: keyring, compositor, browser.
pub fn three_capsules() -> Vec<[u8; ENTRY_LEN]> {
    vec![entry(12, 0xAA, KEYRING, 0), entry(14, 0xBB, COMPOSITOR, 0), entry(31, 0xCC, BROWSER, 0)]
}
