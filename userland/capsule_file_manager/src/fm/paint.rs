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
use super::paint_clip::clip;
use super::paint_footer::paint_footer;
use super::paint_header::paint_header;
use super::paint_rows::paint_rows;
use super::preview_paint::paint_preview;
use super::state::{Mode, State};
use super::theme::{BACKGROUND, FOREGROUND, MUTED};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    if matches!(state.mode, Mode::Help) {
        paint_help(fb);
        return;
    }
    if let (Mode::Preview, Some(preview)) = (state.mode, state.preview.as_ref()) {
        paint_preview(preview, fb);
        return;
    }
    fb.clear(BACKGROUND);
    fb.text(super::paint_metrics::TEXT_LEFT, 18, b"file_manager", FOREGROUND);
    fb.text(super::paint_metrics::TEXT_LEFT, 38, clip(state.prefix.as_bytes(), 40), MUTED);
    paint_header(state, fb);
    paint_rows(state, fb);
    paint_footer(state, fb);
}
