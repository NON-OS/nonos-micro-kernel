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

use super::constants::LINE_HEIGHT;
use crate::term::block::Status;
use crate::term::dimensions::VISIBLE_ROWS;
use crate::term::state::State;
use crate::term::theme::{BLOCK_ERR, BLOCK_OK, BLOCK_RUN, BLOCK_TINT_A, BLOCK_TINT_B};

const STRIPE_W: u32 = 3;
const STRIPE_GAP: u32 = 8;

pub fn draw_block_chrome(state: &State, fb: &mut PaintBuffer, ox: u32, oy: u32, max_y: u32) {
    let g = &state.scrollback.grid;
    for row in 0..VISIBLE_ROWS {
        let y = oy + row as u32 * LINE_HEIGHT;
        if y + LINE_HEIGHT > max_y {
            break;
        }
        let abs = g.abs_of_visible_row(row);
        let (idx, status) = match block_for(state, abs) {
            Some(v) => v,
            None => continue,
        };
        let tint = if idx % 2 == 0 { BLOCK_TINT_A } else { BLOCK_TINT_B };
        fb.fill_rect(ox, y, fb.width.saturating_sub(ox * 2), LINE_HEIGHT, tint);
        let stripe = match status {
            Status::Ok => BLOCK_OK,
            Status::Err => BLOCK_ERR,
            Status::Running => BLOCK_RUN,
        };
        fb.fill_rect(ox.saturating_sub(STRIPE_GAP), y, STRIPE_W, LINE_HEIGHT, stripe);
    }
}

fn block_for(state: &State, abs: u64) -> Option<(usize, Status)> {
    let b = state.block_at(abs)?;
    let idx = state.blocks.iter().position(|x| x.start_abs == b.start_abs)?;
    Some((idx, b.status))
}
