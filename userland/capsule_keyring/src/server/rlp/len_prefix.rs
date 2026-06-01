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

use alloc::vec;
use alloc::vec::Vec;

use super::minimal_be::minimal_be;

pub fn len_prefix(base: u8, len: usize) -> Vec<u8> {
    if len <= 55 {
        return vec![base + len as u8];
    }
    let be = minimal_be(len);
    let mut out = vec![base + 55 + be.len() as u8];
    out.extend_from_slice(&be);
    out
}
