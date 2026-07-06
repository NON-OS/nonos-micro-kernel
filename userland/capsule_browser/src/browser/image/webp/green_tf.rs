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

// Undo the subtract-green transform: the encoder subtracted the green channel
// from red and blue to decorrelate them, so add it back, wrapping each byte.
pub(super) fn apply(px: &mut [u32]) {
    for p in px.iter_mut() {
        let argb = *p;
        let green = (argb >> 8) & 0xff;
        let red = (((argb >> 16) & 0xff) + green) & 0xff;
        let blue = ((argb & 0xff) + green) & 0xff;
        *p = (argb & 0xff00_ff00) | (red << 16) | blue;
    }
}
