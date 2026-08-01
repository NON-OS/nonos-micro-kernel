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

//! Reading the `0x`-prefixed hex the JSON-RPC uses.
//!
//! Everything here rejects on the first thing it does not understand: an odd
//! length, a stray non-hex character, a value that does not fit. A receipt is
//! attacker-influenced, so a malformed field must fail, never wrap or truncate
//! into a number that looks like a payment.

/// One hex digit to its value, or `None` if it is not a hex digit.
fn nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

/// Strip a leading `0x` if present. The RPC always writes one, but accepting
/// its absence costs nothing and never loosens a check.
fn strip_prefix(s: &[u8]) -> &[u8] {
    if s.len() >= 2 && s[0] == b'0' && (s[1] == b'x' || s[1] == b'X') {
        &s[2..]
    } else {
        s
    }
}

/// Decode exactly `N` bytes of hex into `out`, filling from the right so a
/// shorter value is left-zero-padded the way an EVM word is. Fails if the hex
/// has an odd length, holds a non-hex character, or is wider than `N` bytes.
pub fn decode_fixed<const N: usize>(s: &[u8], out: &mut [u8; N]) -> bool {
    let s = strip_prefix(s);
    if !s.len().is_multiple_of(2) || s.len() / 2 > N {
        return false;
    }
    *out = [0u8; N];
    let bytes = s.len() / 2;
    let start = N - bytes;
    for i in 0..bytes {
        let hi = match nibble(s[i * 2]) {
            Some(v) => v,
            None => return false,
        };
        let lo = match nibble(s[i * 2 + 1]) {
            Some(v) => v,
            None => return false,
        };
        out[start + i] = (hi << 4) | lo;
    }
    true
}

/// Decode a hex quantity into a `u128`. A value wider than 16 bytes cannot be
/// held, so it saturates to `u128::MAX`: a payment that large is still at least
/// any real price, and saturating keeps the comparison honest rather than
/// wrapping a huge number down to a small one. Returns `None` on malformed hex.
pub fn decode_u128(s: &[u8]) -> Option<u128> {
    let s = strip_prefix(s);
    if s.is_empty() || !s.len().is_multiple_of(2) {
        return None;
    }
    let bytes = s.len() / 2;
    // More than 16 significant bytes only fits if the high ones are zero.
    let mut acc: u128 = 0;
    for i in 0..bytes {
        let hi = nibble(s[i * 2])?;
        let lo = nibble(s[i * 2 + 1])?;
        let byte = (hi << 4) | lo;
        if bytes - i > 16 && byte != 0 {
            return Some(u128::MAX);
        }
        acc = (acc << 8) | byte as u128;
    }
    Some(acc)
}
