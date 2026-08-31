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

use crate::settings::schema::blocks_for;
use crate::settings::schema::rows::Row;
use crate::settings::state::State;

use super::control_geom::in_control;
use super::metrics::PANE_PAD_X;
use super::walk::{walk, Item};

/// What a click in the pane landed on. `Field` carries the row's index among the
/// section's editable rows, so the caller can move the cursor there.
pub enum Hit {
    Field { index: usize, control: bool },
    Network(usize),
    None,
}

/// `x`/`y` are pane-local, before the scroll offset is applied.
pub fn at(state: &State, x: i32, y: i32, scroll: u32, view_w: u32) -> Hit {
    let card_x = PANE_PAD_X;
    let card_w = view_w.saturating_sub(PANE_PAD_X * 2);
    let target = y + scroll as i32;
    let blocks = blocks_for(state.section);
    let mut index = 0usize;
    let mut hit = Hit::None;
    walk(state, |ry, h, item| {
        let inside = target >= ry as i32 && target < (ry + h) as i32;
        match item {
            Item::Row(bi, ri) => {
                if !matches!(blocks[bi].rows[ri], Row::Field(_)) {
                    return;
                }
                if inside {
                    hit = Hit::Field { index, control: in_control(x, card_x, card_w) };
                }
                index += 1;
            }
            Item::Network(i) if inside => hit = Hit::Network(i),
            _ => {}
        }
    });
    hit
}
