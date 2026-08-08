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

//! Stage 1 descriptor fields.

/// Output address, bits 47:12.
pub const ADDR_MASK: u64 = 0x0000_FFFF_FFFF_F000;

/// There is a descriptor here at all.
pub const VALID: u64 = 1 << 0;
/// A table at levels 0 to 2, a page at level 3. Clear at levels 1 and 2 means
/// a block, so a block is the absence of a flag rather than a flag.
pub const TABLE_OR_PAGE: u64 = 1 << 1;
/// AP[1]: EL0 may reach this mapping.
pub const AP_EL0: u64 = 1 << 6;
/// AP[2]: read-only. Write permission is the absence of this bit.
pub const AP_READ_ONLY: u64 = 1 << 7;
/// SH[1:0] inner shareable.
pub const SH_INNER: u64 = 0b11 << 8;
/// The access flag. Not optional: a leaf without it faults on first touch
/// unless hardware update is enabled, which this kernel does not rely on.
pub const AF: u64 = 1 << 10;
/// This entry belongs to one ASID.
pub const NOT_GLOBAL: u64 = 1 << 11;
/// Privileged execute never.
pub const PXN: u64 = 1 << 53;
/// Unprivileged execute never.
pub const UXN: u64 = 1 << 54;
/// APTable[0]: this table denies EL0 below it.
pub const APTABLE_NO_EL0: u64 = 1 << 61;
