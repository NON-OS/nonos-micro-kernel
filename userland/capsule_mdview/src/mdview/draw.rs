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

use super::layout::{indent, line_height, px, Line, Style};
use super::theme;

pub fn draw_line(fb: &mut PaintBuffer, line: &Line, y: i32) {
    let height = line_height(line.style);
    if line.style == Style::Code {
        let band = fb.width.saturating_sub(2 * theme::MARGIN as u32 - 16);
        let left = (theme::MARGIN - 8).max(0) as u32;
        fb.fill_rect(left, y as u32, band, height as u32, theme::CODE_BG);
    }
    let mut x = theme::MARGIN + indent(line.style);
    if line.style == Style::Bullet && line.lead {
        fb.fill_rect((x - 14) as u32, (y + height / 2 - 2) as u32, 4, 4, theme::ACCENT);
    }
    let size = px(line.style);
    let colour = theme::colour(line.style);
    for span in &line.spans {
        x = if span.mono {
            fb.text_ttf_mono(x, y, &span.text, colour, size)
        } else {
            fb.text_ttf(x, y, &span.text, colour, size)
        };
    }
}
