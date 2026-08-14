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

/// Domains a unit supports, from ND: 2^(4 + 2*ND). ND of 7 is reserved.
pub const fn domain_count(cap: u64) -> u32 {
    let nd = (cap & 0x7) as u32;
    if nd >= 7 {
        0
    } else {
        1u32 << (4 + 2 * nd)
    }
}

/// Widest input address the unit accepts. MGAW stores width - 1.
pub const fn max_address_width(cap: u64) -> u8 {
    (((cap >> 16) & 0x3F) as u8) + 1
}
