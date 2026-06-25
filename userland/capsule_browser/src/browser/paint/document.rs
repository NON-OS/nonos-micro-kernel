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

use crate::browser::state::State;

const TOP: i32 = 40;
const BOTTOM: i32 = 676;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let Some(doc) = &state.document else {
        return;
    };
    for line in &doc.lines {
        let sy = line.y as i32 + TOP - state.scroll as i32;
        if sy < TOP || sy + 18 > BOTTOM {
            continue;
        }
        for s in &line.spans {
            fb.text(s.x, sy as u32, s.text.as_bytes(), s.color);
            if s.href.is_some() {
                fb.fill_rect(s.x, sy as u32 + 16, s.w, 1, s.color);
            }
        }
    }
}
