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

// Extract the 32-byte ABI word at `index` from an eth_call result. A tuple of
// static uint256 values is returned as consecutive 32-byte words, so a stats
// call packs each field at a fixed word offset.
pub fn parse_call_word(resp: &[u8], index: usize) -> Option<[u8; 32]> {
    let value = super::find_result::find_result(resp)?;
    let hex = value.strip_prefix(b"0x")?;
    let start = index.checked_mul(64)?;
    let end = start.checked_add(64)?;
    if hex.len() < end {
        return None;
    }
    let mut out = [0u8; 32];
    let mut i = 0;
    while i < 32 {
        let hi = nibble(hex[start + i * 2])?;
        let lo = nibble(hex[start + i * 2 + 1])?;
        out[i] = (hi << 4) | lo;
        i += 1;
    }
    Some(out)
}

fn nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
