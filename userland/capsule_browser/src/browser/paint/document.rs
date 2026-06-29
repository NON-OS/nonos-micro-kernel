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

use crate::browser::manifest::WIDTH;
use crate::browser::state::State;

const TOP: i32 = 80;
const BOTTOM: i32 = 676;
pub const VIEW_H: u32 = (BOTTOM - TOP) as u32;
const PAGE_BG: u32 = 0xFF18_1B20;
const FG: u32 = 0xFFE8_EAED;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.fill_rect(0, TOP as u32, WIDTH, (BOTTOM - TOP) as u32, PAGE_BG);
    let Some(doc) = &state.document else {
        fb.text(16, TOP as u32 + 24, state.status.as_bytes(), FG);
        return;
    };
    for line in &doc.lines {
        let sy = line.y as i32 + TOP - state.scroll as i32;
        if sy < TOP || sy + line.height as i32 > BOTTOM {
            continue;
        }
        for s in &line.spans {
            fb.text_scaled(s.x, sy as u32, s.text.as_bytes(), s.color, s.scale);
            if s.bold {
                fb.text_scaled(s.x + 1, sy as u32, s.text.as_bytes(), s.color, s.scale);
            }
            if s.href.is_some() {
                fb.fill_rect(s.x, sy as u32 + line.height - 4, s.w, 1, s.color);
            }
        }
    }
}
