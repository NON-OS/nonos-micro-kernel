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

use super::help::paint_help;
use super::paint_footer::paint_footer;
use super::paint_grid::paint_grid;
use super::paint_header::paint_header;
use super::paint_info::paint_info;
use super::paint_rows::paint_rows;
use super::preview_paint::paint_preview;
use super::screen::Screen;
use super::screen_home::paint_home;
use super::screen_recents::paint_recents;
use super::screen_search::paint_search;
use super::screen_shared::paint_shared;
use super::screen_tags::paint_tags;
use super::sel_band::paint_band;
use super::state::{Mode, State, ViewKind};
use super::chrome_card::Plate;
use super::chrome_glow::glow_in;
use super::theme::{HAIR, R_SHELL, WIN};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    if matches!(state.mode, Mode::Help) {
        paint_help(fb);
        return;
    }
    if let (Mode::Preview, Some(preview)) = (state.mode, state.preview.as_ref()) {
        paint_preview(preview, fb);
        return;
    }
    let (w, h) = (fb.width, fb.height);
    fb.clear(0);
    Plate::new(WIN).radius(R_SHELL).line(HAIR).draw(fb, 0, 0, w, h);
    glow_in(fb, 0, 0, w, h, R_SHELL, 6);
    super::paint_sidebar::paint_sidebar(state, fb);
    paint_header(state, fb);
    match state.screen {
        Screen::Browse => paint_browse(state, fb),
        Screen::Home => paint_home(state, fb),
        Screen::Recents => paint_recents(state, fb),
        Screen::Search => paint_search(state, fb),
        Screen::Tags => paint_tags(state, fb),
        Screen::Shared => paint_shared(state, fb),
    }
    paint_footer(state, fb);
}

// The directory listing plus the info panel describing what the cursor is on.
// The presentation toggle applies only to the listing half; the panel's strip is
// reserved out of the content width, so the two never overlap.
fn paint_browse(state: &State, fb: &mut PaintBuffer) {
    match state.view {
        ViewKind::Grid => paint_grid(state, fb),
        ViewKind::List => paint_rows(state, fb),
    }
    paint_info(state, fb);
    paint_band(state, fb);
}
