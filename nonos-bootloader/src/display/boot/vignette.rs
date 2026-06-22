// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

// Per-channel color interpolation, shared by the progress gradient.
pub(crate) fn lerp(a: u32, b: u32, t: u32) -> u32 {
    let mut out = 0xFF00_0000;
    let mut s = 0u32;
    while s < 24 {
        let c = (((a >> s) & 0xFF) * (256 - t) + ((b >> s) & 0xFF) * t) / 256;
        out |= (c & 0xFF) << s;
        s += 8;
    }
    out
}
