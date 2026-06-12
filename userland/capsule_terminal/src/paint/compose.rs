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

use super::constants::{BODY_TOP, FOOTER_H, LINE_HEIGHT, TEXT_LEFT};
use super::draw_input_line::draw_input_line;
use super::fetch::draw_fetch;
use super::footer::draw_footer;
use super::header::draw_header;
use crate::term::scrollback::Role;
use crate::term::state::State;
use crate::term::theme::{BACKGROUND, ERROR, FOREGROUND};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.clear(BACKGROUND);
    draw_header(state, fb);
    let input_y = fb.height.saturating_sub(FOOTER_H + LINE_HEIGHT);
    if state.fresh {
        draw_fetch(state, fb);
    } else {
        let mut y = BODY_TOP;
        for (row, role) in state.scrollback.visible().rows() {
            if y + LINE_HEIGHT > input_y {
                break;
            }
            let color = match role {
                Role::Error => ERROR,
                Role::Normal => FOREGROUND,
            };
            fb.text(TEXT_LEFT, y, row, color);
            y += LINE_HEIGHT;
        }
    }
    draw_input_line(state, fb, input_y);
    draw_footer(fb);
}
