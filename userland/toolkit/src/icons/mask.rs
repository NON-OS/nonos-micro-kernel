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

use super::id::IconId;
use super::table::MASKS;

pub fn mask(id: IconId) -> &'static [u8] {
    MASKS[id as usize]
}

/// Side length of `mask(id)`, recovered from its length rather than stored, so
/// regenerating the assets at another resolution needs no code change.
pub fn dim(id: IconId) -> u32 {
    (mask(id).len() as u32).isqrt()
}
