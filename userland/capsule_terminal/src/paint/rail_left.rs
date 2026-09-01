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

use super::rail_left_geom::{sections, Sections};
use super::rail_row::{base_name, row_rect, rows_fit};
use super::rail_row_draw::draw_row;
use super::rail_text::head;
use super::tab_label::session_name;
use super::tokens::DOT_IDLE;
use crate::layout::Rect;
use crate::term::prefs::types::Project;
use crate::term::state::State;
use crate::term::theme::types::Theme;

/// The navigation rail: the sessions this window owns and the paths the user
/// has pinned. Both lists are the same row shape, so a click resolves through
/// one geometry no matter which section it landed in.
pub fn draw(
    fb: &mut PaintBuffer,
    r: Rect,
    tabs: &[State],
    active: usize,
    projects: &[Project],
    t: &Theme,
) {
    fb.blend_rect(r.x + r.w.saturating_sub(1), r.y, 1, r.h, t.chrome_edge);
    let s = sections(r, tabs.len() as u32);
    caption(fb, &s, t);
    let mut buf = [0u8; 32];
    for i in 0..(tabs.len() as u32).min(rows_fit(s.s_list)) {
        let tab = &tabs[i as usize];
        let n = session_name(i as usize, &mut buf);
        let name = core::str::from_utf8(&buf[..n]).unwrap_or("");
        let sub = core::str::from_utf8(tab.cwd.as_bytes()).unwrap_or("");
        let dot = if tab.fg_running { t.run } else { DOT_IDLE };
        draw_row(fb, row_rect(i, s.s_list), dot, name, sub, i as usize == active, t);
    }
    for i in 0..(projects.len() as u32).min(rows_fit(s.p_list)) {
        let path = projects[i as usize].as_str();
        draw_row(fb, row_rect(i, s.p_list), DOT_IDLE, base_name(path), path, false, t);
    }
}

fn caption(fb: &mut PaintBuffer, s: &Sections, t: &Theme) {
    head(fb, s.s_head.x, s.s_head.y, s.s_head.w, "SESSIONS", t);
    plus(fb, s.s_plus, t);
    head(fb, s.p_head.x, s.p_head.y, s.p_head.w, "PROJECTS", t);
    plus(fb, s.p_plus, t);
}

fn plus(fb: &mut PaintBuffer, r: Rect, t: &Theme) {
    let arm = r.w / 2;
    let cx = r.x + r.w / 2;
    let cy = r.y + r.h / 2;
    fb.blend_rect(cx.saturating_sub(arm / 2), cy, arm, 1, t.dim);
    fb.blend_rect(cx, cy.saturating_sub(arm / 2), 1, arm, t.dim);
}
