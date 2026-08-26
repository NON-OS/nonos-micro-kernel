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
use nonos_toolkit::font::ttf::line_height;

use super::metrics::{BODY_PX, CARD_PAD_X, NOTE_PX};
use super::text;
use super::theme::{LABEL_FG, SUBLABEL_FG};

/// Lays out a row's title and optional sub-label. A row with a note puts its
/// title above the vertical centre so the pair reads as one block.
pub fn paint(
    fb: &mut PaintBuffer,
    x: u32,
    screen_y: i32,
    row_h: u32,
    title: &str,
    note: Option<&str>,
) {
    let lx = x + CARD_PAD_X;
    match note {
        None => {
            let top = text::centred_top(0, row_h, BODY_PX) + screen_y;
            text::left(fb, lx, top, title, LABEL_FG, BODY_PX);
        }
        Some(n) => {
            let top = screen_y + 11;
            text::left(fb, lx, top, title, LABEL_FG, BODY_PX);
            text::left(fb, lx, top + line_height(BODY_PX) + 1, n, SUBLABEL_FG, NOTE_PX);
        }
    }
}
