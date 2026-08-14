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

/// An overflow, and at least one pending fault record.
pub const FSTS_PFO: u32 = 1 << 0;
pub const FSTS_PPF: u32 = 1 << 1;

/// High half of a fault record. The record's position comes from
/// `cap::fault_recording_offset`.
pub const FRCD_FAULT: u64 = 1 << 63;
pub const FRCD_TYPE_READ: u64 = 1 << 62;

pub const fn frcd_reason(high: u64) -> u8 {
    ((high >> 32) & 0xFF) as u8
}

pub const fn frcd_source(high: u64) -> u16 {
    (high & 0xFFFF) as u16
}
