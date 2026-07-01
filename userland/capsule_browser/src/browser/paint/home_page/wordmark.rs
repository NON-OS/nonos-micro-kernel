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
use crate::browser::paint::home_page::constants;

pub fn wordmark(fb: &mut PaintBuffer) {
    let advance = fb.glyph_advance().saturating_mul(3);
    let width = advance.saturating_mul(5).min(WIDTH);
    let x0 = (WIDTH - width) / 2;
    let y = 108;
    fb.text_scaled(x0, y, b"N", constants::FG, 3);
    fb.text_scaled(x0 + advance, y, b"\xd8", constants::ACCENT, 3);
    fb.text_scaled(x0 + advance * 2, y, b"NOS", constants::FG, 3);
}
