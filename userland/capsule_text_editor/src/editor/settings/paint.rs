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

//! The Settings screen: an opaque ground, the navigation rail, and the panel
//! for the selected section. General has its own hand-laid card; the other six
//! sections share one table-driven painter.

use nonos_app_skeleton::PaintBuffer;

use super::super::app::Editor;
use super::super::theme;
use super::pane::paint_pane;
use super::rail::paint_rail;
use super::sect::section;
use super::sect_paint::paint_section;
use super::state::{latch_width, state};

pub(crate) fn paint_settings(_ed: &mut Editor, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    fb.fill_rect(0, 0, w, h, theme::active().background);
    latch_width(w);
    let st = state();
    paint_rail(fb, st.nav);
    if st.nav == 0 {
        paint_pane(fb, &st);
    } else if let Some(sec) = section(st.nav) {
        paint_section(fb, st.nav, sec);
    }
}
