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

/// POLYVAL state, RFC 8452 section 3.
///
/// This is GHASH's field seen the other way round: POLYVAL works in a
/// little-endian bit order over the reversed polynomial, so the reduction
/// constant is 0xC200..0 applied on a right shift rather than 0xE100..0 on a
/// left shift. Mixing the two conventions produces a tag that looks plausible
/// and authenticates nothing.
pub struct Polyval {
    pub(super) h: [u64; 2],
    pub(super) acc: [u64; 2],
}
