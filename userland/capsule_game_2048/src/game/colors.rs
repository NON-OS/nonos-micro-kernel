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

pub fn tile_color(v: u64) -> u32 {
    match v {
        0 => 0xFF2A2A3E,
        2 => 0xFFEEE4DA,
        4 => 0xFFEDE0C8,
        8 => 0xFFF2B179,
        16 => 0xFFF59563,
        32 => 0xFFF67C5F,
        64 => 0xFFF65E3B,
        128 => 0xFFEDCF72,
        256 => 0xFFEDCC61,
        512 => 0xFFEDC850,
        1024 => 0xFFEDC53F,
        _ => 0xFFEDC22E,
    }
}
