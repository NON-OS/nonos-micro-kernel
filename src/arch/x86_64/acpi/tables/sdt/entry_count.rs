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

/// A generous upper bound on a single ACPI table's declared length. Real tables
/// are far smaller; capping the walk of a length-delimited table (MADT, SRAT) at
/// this bounds a firmware-declared length that would otherwise scan far past the
/// table's real extent.
pub const MAX_TABLE_BYTES: u64 = 1 << 20;

/// Number of fixed-size entries packed after a table's fixed-size header.
///
/// A malformed table can declare a `length` smaller than its own header. A raw
/// subtraction there underflows into a near-2^64 count that then drives an
/// out-of-bounds read loop over firmware-controlled memory. Saturating the
/// subtraction yields zero entries for such a table instead.
pub fn sdt_entry_count(table_length: usize, header_size: usize, entry_size: usize) -> usize {
    if entry_size == 0 {
        return 0;
    }
    table_length.saturating_sub(header_size) / entry_size
}
