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

use super::draw::draw_line;
use super::layout::{gap, line_height, Line};
use super::theme;

const TOP: i32 = 20;

pub fn paint(fb: &mut PaintBuffer, lines: &[Line], error: Option<&'static str>) {
    fb.clear(theme::BG);
    fb.fill_rect(0, 0, fb.width, 4, theme::ACCENT);
    if let Some(message) = error {
        fb.text_ttf(theme::MARGIN, TOP, message, theme::FAIL, 15.0);
        fb.text_ttf(theme::MARGIN, TOP + 26, "esc closes this window", theme::DIM, 13.0);
        return;
    }
    let mut y = TOP;
    let mut started = false;
    for line in lines {
        if line.lead && started {
            y += gap(line.style);
        }
        let height = line_height(line.style);
        if y + height > fb.height as i32 {
            break;
        }
        draw_line(fb, line, y);
        y += height;
        started = true;
    }
}
