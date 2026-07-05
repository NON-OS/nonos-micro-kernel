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

use super::bitread::BitReader;

pub(super) const SIGNATURE: u32 = 0x2f;

// VP8L stream header: a signature byte, then 14-bit width-1 and height-1,
// an alpha flag and a 3-bit version. Returns (width, height).
pub(super) fn read_header(br: &mut BitReader) -> Option<(u32, u32)> {
    if br.read(8) != SIGNATURE {
        return None;
    }
    let w = br.read(14) + 1;
    let h = br.read(14) + 1;
    let _alpha = br.read(1);
    let version = br.read(3);
    if version != 0 || br.eos {
        return None;
    }
    Some((w, h))
}
