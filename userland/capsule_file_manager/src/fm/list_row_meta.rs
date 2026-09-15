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

use super::entries::Entry;
use super::file_kind::kind_of;
use super::fmt_time::fmt_time;
use super::human_size::human_size;
use super::icon_draw::draw;
use super::icon_path::Icon;
use super::layout::ROW_H;
use super::list_cols::{Cols, MENU_W};
use super::list_kind_label::kind_label;
use super::measure_text::right_text;
use super::theme::INK3;

const META_PX: f32 = 14.0;
const MENU_S: u32 = 16;
const TEXT_DY: u32 = 15;

/// The three right-aligned meta cells and the per-row menu affordance. Every
/// cell right-aligns on its own column's end, the same edge `list_head` set its
/// label against.
///
/// The ellipsis is drawn in the tertiary ink because nothing is wired behind it:
/// this capsule has no row-menu machinery, and the house pattern is that an
/// unwired control reads as dimmed rather than silently doing nothing.
pub fn row_meta(fb: &mut PaintBuffer, entry: &Entry, y: u32, c: &Cols) {
    let ty = y + TEXT_DY;
    right_text(fb, c.cols[1].x + c.cols[1].w, ty, kind_label(kind_of(entry)), META_PX, INK3);
    if let Some(size) = entry.size {
        right_text(fb, c.cols[2].x + c.cols[2].w, ty, &human_size(size), META_PX, INK3);
    }
    if entry.mtime != 0 {
        right_text(fb, c.cols[3].x + c.cols[3].w, ty, &fmt_time(entry.mtime), META_PX, INK3);
    }
    let mx = c.menu_x + MENU_W.saturating_sub(MENU_S) / 2;
    draw(fb, Icon::Ellipsis, mx, y + (ROW_H - MENU_S) / 2, MENU_S, INK3);
}
