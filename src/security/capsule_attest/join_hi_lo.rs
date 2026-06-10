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

use super::field::field;

pub(super) fn join_hi_lo(pubins: &[u8], hi_idx: usize) -> Option<[u8; 32]> {
    let hi = field(pubins, hi_idx)?;
    let lo = field(pubins, hi_idx + 1)?;
    let mut out = [0u8; 32];
    out[0..16].copy_from_slice(&hi[16..32]);
    out[16..32].copy_from_slice(&lo[16..32]);
    Some(out)
}
