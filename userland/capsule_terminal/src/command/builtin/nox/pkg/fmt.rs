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

use super::names::CAP_NAMES;
use crate::term::util::format_u64;

// Trust tier as the kernel's manifest summary encodes it: 1 for a capsule
// enrolled in the local trust store, 2 for one carrying a publisher
// identity certificate.
pub(super) fn tier_word(tier: u8) -> &'static [u8] {
    match tier {
        1 => b"enrolled",
        2 => b"publisher",
        _ => b"unknown",
    }
}

// Spell out every capability the package asks for. A bit above the known
// table is a capability this build does not name yet, so it is shown as
// `bit<N>`: a consent prompt must never omit something that was granted.
pub(super) fn caps_line(caps: u64) -> Vec<u8> {
    let mut out = Vec::new();
    for i in 0..64 {
        if caps & (1u64 << i) == 0 {
            continue;
        }
        if !out.is_empty() {
            out.push(b' ');
        }
        match CAP_NAMES.get(i) {
            Some(name) => out.extend_from_slice(name),
            None => push_unnamed(&mut out, i),
        }
    }
    if out.is_empty() {
        out.extend_from_slice(b"(none)");
    }
    out
}

fn push_unnamed(out: &mut Vec<u8>, bit: usize) {
    let mut num = [0u8; 24];
    let k = format_u64(bit as u64, &mut num);
    out.extend_from_slice(b"bit");
    out.extend_from_slice(&num[..k]);
}

// The first eight bytes of the package digest, enough for a human to match
// the consent prompt against a published hash.
pub(super) fn digest_prefix(d: &[u8; 32]) -> Vec<u8> {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = Vec::with_capacity(16);
    for b in &d[..8] {
        out.push(HEX[(b >> 4) as usize]);
        out.push(HEX[(b & 15) as usize]);
    }
    out
}
