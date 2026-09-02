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

use super::rail_band::visible;
use super::rail_caption::caption;
use super::rail_left_geom::nav_sections;
use super::rail_row::{base_name, row_band};
use super::rail_row_draw::draw_row;
use super::rail_scroll::{clamp, telemetry_top, RailFit};
use super::rail_telemetry;
use super::rail_text::RAIL_PAD;
use super::tab_label::session_name;
use super::tokens::DOT_IDLE;
use crate::layout::Rect;
use crate::rail::Rail;
use crate::term::prefs::types::Project;
use crate::term::state::State;
use crate::term::theme::types::Theme;

/// The one rail, painted as a single scrolled column: the sessions this window
/// owns, the paths the user has pinned, then the machine telemetry. Everything
/// is drawn into a sub-buffer covering the rail, so a band scrolled past either
/// edge is cut by the buffer instead of bleeding into the rest of the chrome.
pub fn draw(
    fb: &mut PaintBuffer,
    r: Rect,
    tabs: &[State],
    active: usize,
    projects: &[Project],
    rail: &Rail,
    telemetry: bool,
    scroll: u32,
    t: &Theme,
) {
    if r.w == 0 || r.h == 0 {
        return;
    }
    let fit = RailFit {
        sessions: tabs.len() as u32,
        projects: projects.len() as u32,
        telemetry,
    };
    let mut fb = fb.sub(r.x, r.y, r.w, r.h);
    let fb = &mut fb;
    fb.blend_rect(r.w.saturating_sub(1), 0, 1, r.h, t.chrome_edge);
    let off = clamp(scroll, fit, r.h);
    let s = nav_sections(r, off, fit.sessions, fit.projects);
    caption(fb, &s, t);
    let mut buf = [0u8; 32];
    for i in 0..fit.sessions {
        let band = row_band(i, &s.s_list);
        if !visible(&band, r.h) {
            continue;
        }
        let tab = &tabs[i as usize];
        let n = session_name(i as usize, &mut buf);
        let name = core::str::from_utf8(&buf[..n]).unwrap_or("");
        let sub = core::str::from_utf8(tab.cwd.as_bytes()).unwrap_or("");
        let dot = if tab.fg_running { t.run } else { DOT_IDLE };
        draw_row(fb, band, dot, name, sub, i as usize == active, t);
    }
    for i in 0..fit.projects {
        let band = row_band(i, &s.p_list);
        if !visible(&band, r.h) {
            continue;
        }
        let path = projects[i as usize].as_str();
        draw_row(fb, band, DOT_IDLE, base_name(path), path, false, t);
    }
    if telemetry {
        let y = telemetry_top(off, fit);
        let w = r.w.saturating_sub(RAIL_PAD * 2);
        rail_telemetry::draw(fb, RAIL_PAD, y, w, r.h, rail, t);
    }
}
