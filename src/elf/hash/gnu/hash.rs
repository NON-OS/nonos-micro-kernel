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

pub const GNU_HASH_BLOOM_SHIFT: u32 = 6;

pub fn gnu_hash(name: &[u8]) -> u32 {
    let mut hash = 5381u32;
    for &byte in name {
        if byte == 0 {
            break;
        }
        hash = hash.wrapping_mul(33).wrapping_add(byte as u32);
    }
    hash
}
