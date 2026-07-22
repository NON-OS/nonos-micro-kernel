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

use super::poly_mod::poly_mod;

/// 18-bit BCH version code (versions >= 7): 6 data bits under generator 0x1F25.
pub(super) fn version_bits(version: u8) -> u32 {
    let data = version as u32;
    let bch = poly_mod(data << 12, 0x1F25);
    (data << 12) | bch
}
