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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacKey {
    pub lo: u64,
    pub hi: u64,
}

impl PacKey {
    pub const fn new(lo: u64, hi: u64) -> Self {
        Self { lo, hi }
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        let lo = u64::from_le_bytes(first_word(bytes));
        let hi = u64::from_le_bytes(second_word(bytes));
        Self { lo, hi }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacKeys {
    pub ia: PacKey,
    pub ib: PacKey,
    pub da: PacKey,
    pub db: PacKey,
    pub ga: PacKey,
}

fn first_word(bytes: [u8; 16]) -> [u8; 8] {
    [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]
}

fn second_word(bytes: [u8; 16]) -> [u8; 8] {
    [bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]]
}
