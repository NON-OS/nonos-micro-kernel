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

pub(super) fn mix(bg: u32, fg: u32, a: u32) -> u32 {
    let a = a.min(255);
    let ia = 255 - a;
    let br = (bg >> 16) & 0xFF;
    let bgc = (bg >> 8) & 0xFF;
    let bb = bg & 0xFF;
    let fr = (fg >> 16) & 0xFF;
    let fgc = (fg >> 8) & 0xFF;
    let fb = fg & 0xFF;
    let r = (br * ia + fr * a) / 255;
    let g = (bgc * ia + fgc * a) / 255;
    let b = (bb * ia + fb * a) / 255;
    0xFF00_0000 | (r << 16) | (g << 8) | b
}
