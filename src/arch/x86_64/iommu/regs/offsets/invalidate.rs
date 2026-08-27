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

pub const CCMD: usize = 0x028;
pub const CCMD_ICC: u64 = 1 << 63;
pub const CCMD_CIRG_GLOBAL: u64 = 1 << 61;

/// The IOTLB registers have no fixed offset; ECAP carries their position in
/// 16-byte units.
pub const fn iva_offset(ecap: u64) -> usize {
    (((ecap >> 8) & 0x3FF) as usize) * 16
}

pub const fn iotlb_offset(ecap: u64) -> usize {
    iva_offset(ecap) + 8
}

pub const IOTLB_IVT: u64 = 1 << 63;
pub const IOTLB_IIRG_GLOBAL: u64 = 1 << 60;
/// The granularity the unit actually performed. Zero means it did nothing.
pub const IOTLB_IAIG_MASK: u64 = 0b11 << 57;
