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

//! The Browse info panel: what the cursor is on, described, and the controls
//! that act on it. Its strip is reserved out of the content width by
//! `layout::info_x`, the same function the list and the grid subtract, so the
//! panel can never overlap a row. Every band it draws is placed by `info_geom`,
//! which `info_hit` reads back, so no control is drawn where a click is not
//! looked for.

use nonos_app_skeleton::PaintBuffer;

use super::info_cache::cached_dir;
use super::info_dir::dir_body;
use super::info_file::file_body;
use super::info_geom::info_geom;
use super::info_perms::paint_perms;
use super::info_quick_paint::paint_quick;
use super::info_sizes::{PAD, SUB_PX, TITLE_PX};
use super::info_tags::tag_band;
use super::layout::{info_x, FIRST_ROW_Y, FOOTER_H, INFO_W};
use super::measure_text::truncate_to_width;
use super::recents_group::parent_of;
use super::state::State;
use super::theme::{INK, INK3, LINE, PANEL};

pub fn paint_info(state: &State, fb: &mut PaintBuffer) {
    let x = info_x(fb.width);
    let h = fb.height.saturating_sub(FIRST_ROW_Y + FOOTER_H + 8);
    fb.panel(x, FIRST_ROW_Y, INFO_W, h, 14, PANEL, LINE);
    let (Some(entry), Some(g)) =
        (state.entries.get(state.cursor), info_geom(state, fb.width, fb.height))
    else {
        let pen = (x + PAD) as i32;
        let _ = fb.text_ttf(pen, (FIRST_ROW_Y + 20) as i32, "Nothing selected", INK3, SUB_PX);
        return;
    };
    let title = truncate_to_width(fb, entry.label.trim_end_matches('/'), TITLE_PX, g.w);
    let _ = fb.text_ttf(g.x as i32, g.title_y as i32, title, INK, TITLE_PX);
    let loc = truncate_to_width(fb, parent_of(&entry.full_path), SUB_PX, g.w);
    let _ = fb.text_ttf(g.x as i32, g.loc_y as i32, loc, INK3, SUB_PX);
    fb.blend_rect(g.x, g.rule_y, g.w, 1, LINE);
    paint_quick(fb, &g);
    if entry.is_dir {
        dir_body(fb, cached_dir(state, entry.full_path.as_str()), g.x, g.body_top, g.w);
    } else {
        file_body(fb, entry, g.x, g.body_top, g.w);
    }
    let names = state.tags.tags_for(entry.full_path.as_str());
    tag_band(fb, &g, &names);
    paint_perms(fb, &g, entry);
}
