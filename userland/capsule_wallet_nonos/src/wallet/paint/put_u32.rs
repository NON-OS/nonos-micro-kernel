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

/// Write a number into a slice at an offset, returning how many bytes it
/// took. `format_u32` wants a fixed buffer; the paint code wants to build a
/// line piece by piece, and this is the join between the two.
pub fn put_u32(out: &mut [u8], v: u32) -> usize {
    let mut tmp = [0u8; 10];
    let n = super::format_u32::format_u32(v, &mut tmp);
    let take = n.min(out.len());
    out[..take].copy_from_slice(&tmp[..take]);
    take
}
