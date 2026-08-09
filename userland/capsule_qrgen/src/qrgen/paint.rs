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

use super::code::{Matrix, PAYLOAD};
use super::draw::draw_matrix;
use super::theme;

const TITLE_Y: i32 = 20;
const PAYLOAD_Y: i32 = 48;
const MATRIX_TOP: u32 = 76;

pub fn paint(fb: &mut PaintBuffer, matrix: Option<&Matrix>) {
    fb.clear(theme::BG);
    fb.fill_rect(0, 0, fb.width, 4, theme::ACCENT);
    fb.text_ttf(theme::MARGIN, TITLE_Y, "qr generator", theme::ACCENT, 19.0);
    fb.text_ttf(theme::MARGIN, PAYLOAD_Y, PAYLOAD, theme::TEXT, 14.0);
    let Some(matrix) = matrix else {
        fb.text_ttf(theme::MARGIN, MATRIX_TOP as i32, "encode failed", theme::FAIL, 15.0);
        return;
    };
    let bottom = draw_matrix(fb, matrix, MATRIX_TOP);
    fb.text_ttf(theme::MARGIN, bottom as i32 + 12, "esc closes this window", theme::DIM, 13.0);
}
