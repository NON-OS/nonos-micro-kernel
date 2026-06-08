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

use crate::display::constants::{COLOR_ACCENT, COLOR_ACCENT_DIM};
use crate::display::font::draw_string_2x;
use crate::display::gop::is_initialized;

const WORD: &[u8] = b"N\xd8NOS";

pub fn draw_wordmark(x: u32, y: u32) {
    if !is_initialized() {
        return;
    }
    draw_string_2x(x.saturating_sub(1), y, WORD, COLOR_ACCENT_DIM);
    draw_string_2x(x + 1, y, WORD, COLOR_ACCENT_DIM);
    draw_string_2x(x, y.saturating_sub(1), WORD, COLOR_ACCENT_DIM);
    draw_string_2x(x, y, WORD, COLOR_ACCENT);
}
