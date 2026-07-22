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
use crate::version::Ecc;

/// 15-bit BCH format code: 5 data bits (2 ecc level, 3 mask) under generator
/// 0x537, XORed with the fixed 0x5412 pattern.
pub(super) fn format_bits(ecc: Ecc, mask: u8) -> u16 {
    let data = ((ecc.format_bits() << 3) | mask as u16) as u32;
    let bch = poly_mod(data << 10, 0x537);
    (((data << 10) | bch) ^ 0x5412) as u16
}
