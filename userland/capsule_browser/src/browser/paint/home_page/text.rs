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

use nonos_app_skeleton::PaintBuffer;

use crate::browser::manifest::WIDTH;

pub fn centered_text(fb: &mut PaintBuffer, bytes: &[u8], color: u32, y: u32) {
    let width = (bytes.len() as u32 * fb.glyph_advance()).min(WIDTH);
    let x = (WIDTH - width) / 2;
    fb.text(x, y, bytes, color);
}
