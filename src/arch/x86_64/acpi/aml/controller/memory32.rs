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

/// Decode a Memory32Fixed descriptor payload (ACPI 6.x 6.4.3.4): the base
/// address at offset 1 and the range length at offset 5, both little-endian.
/// A zero base is treated as absent.
pub(super) fn parse_memory32_fixed(data: &[u8]) -> Option<(u32, u32)> {
    let base = u32::from_le_bytes([*data.get(1)?, *data.get(2)?, *data.get(3)?, *data.get(4)?]);
    let len = u32::from_le_bytes([*data.get(5)?, *data.get(6)?, *data.get(7)?, *data.get(8)?]);
    (base != 0).then_some((base, len))
}
