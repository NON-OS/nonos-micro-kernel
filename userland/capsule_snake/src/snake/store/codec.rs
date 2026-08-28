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

pub const VERSION: u16 = 1;
pub const HEADER_LEN: usize = 8;
pub const RUN_LEN: usize = 12;
pub const AWARD_LEN: usize = 2;
pub const MAX_AWARDS: usize = 64;
pub const MAGIC_RUNS: [u8; 4] = *b"SNKR";
pub const MAGIC_AWARDS: [u8; 4] = *b"SNKA";

// Header: magic[4] | version u16 LE | count u16 LE. Everything after it is a
// fixed-width entry, so one length check proves the whole tail is present.
pub fn header(magic: [u8; 4], count: usize, out: &mut Vec<u8>) {
    out.extend_from_slice(&magic);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&(count as u16).to_le_bytes());
}

// Validates the header against this build's layout and returns how many entries
// the caller may read. A short, foreign or over-long record is an Err, never a
// panic: the caller degrades to default career state.
pub fn count_of(
    bytes: &[u8],
    magic: [u8; 4],
    entry: usize,
    max: usize,
) -> Result<usize, &'static str> {
    if bytes.len() < HEADER_LEN {
        return Err("snake store short");
    }
    if bytes[..4] != magic[..] {
        return Err("snake store magic");
    }
    if u16::from_le_bytes([bytes[4], bytes[5]]) != VERSION {
        return Err("snake store version");
    }
    let count = u16::from_le_bytes([bytes[6], bytes[7]]) as usize;
    if count > max {
        return Err("snake store count");
    }
    if bytes.len() < HEADER_LEN + count * entry {
        return Err("snake store truncated");
    }
    Ok(count)
}
