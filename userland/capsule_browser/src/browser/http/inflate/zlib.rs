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

use super::inflate_raw::inflate;

pub fn zlib(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 6 || data[0] & 0x0f != 8 {
        return None;
    }
    if (((data[0] as u16) << 8) | data[1] as u16) % 31 != 0 {
        return None;
    }
    inflate(&data[2..data.len() - 4])
}
