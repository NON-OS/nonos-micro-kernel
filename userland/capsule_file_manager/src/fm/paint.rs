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

use super::state::State;
use super::theme::{BACKGROUND, DIRECTORY, FOREGROUND, MUTED, SELECTED};

const TEXT_LEFT: u32 = 16;
const FIRST_ROW_Y: u32 = 64;
const ROW_HEIGHT: u32 = 22;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.clear(BACKGROUND);
    fb.text(TEXT_LEFT, 18, b"file_manager", FOREGROUND);
    fb.text(TEXT_LEFT, 38, state.prefix.as_bytes(), MUTED);
    fb.text(TEXT_LEFT, 232, state.status, MUTED);
    let mut y = FIRST_ROW_Y;
    for (i, entry) in state.entries.iter().enumerate() {
        let color = if entry.is_dir { DIRECTORY } else { FOREGROUND };
        if i == state.cursor {
            fb.fill_rect(TEXT_LEFT, y - 4, 300, 18, 0x222F5AA0);
        }
        fb.text(TEXT_LEFT, y, entry.label.as_bytes(), if i == state.cursor { SELECTED } else { color });
        y += ROW_HEIGHT;
    }
    if state.entries.is_empty() {
        fb.text(TEXT_LEFT, FIRST_ROW_Y, b"empty directory", MUTED);
    }
    if let Some(path) = state.preview.as_ref() {
        fb.text(TEXT_LEFT, 208, b"selected:", FOREGROUND);
        fb.text(TEXT_LEFT + 88, 208, path.as_bytes(), MUTED);
    }
}
