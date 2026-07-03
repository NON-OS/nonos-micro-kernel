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

use crate::browser::paint::home_page::constants;

pub fn wordmark(fb: &mut PaintBuffer) {
    const PX: f32 = 40.0;
    let y = 100;
    let total = fb.measure_ttf("NØNOS", PX);
    let x0 = (fb.width as i32 - total) / 2;
    // Paint the three runs so the middle glyph can carry the accent color.
    let x1 = fb.text_ttf(x0, y, "N", constants::FG, PX);
    let x2 = fb.text_ttf(x1, y, "Ø", constants::ACCENT, PX);
    fb.text_ttf(x2, y, "NOS", constants::FG, PX);
}
