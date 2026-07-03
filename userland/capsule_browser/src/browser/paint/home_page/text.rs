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

pub fn centered_text(fb: &mut PaintBuffer, bytes: &[u8], color: u32, y: u32) {
    let text = core::str::from_utf8(bytes).unwrap_or("");
    let width = fb.measure_ttf(text, 15.0);
    let x = (fb.width as i32 - width) / 2;
    fb.text_ttf(x, y as i32, text, color, 15.0);
}
