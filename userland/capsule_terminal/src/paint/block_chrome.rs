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

use super::block_meta::draw_meta;
use super::metrics::Metrics;
use super::shade::elevate;
use crate::term::block::Status;
use crate::term::dimensions::VISIBLE_ROWS;
use crate::term::state::State;
use crate::term::theme::{BLOCK_ERR, BLOCK_OK, BLOCK_RUN};

const STRIPE_W: u32 = 3;
const STRIPE_GAP: u32 = 8;

pub fn draw_block_chrome(
    state: &State,
    fb: &mut PaintBuffer,
    ox: u32,
    oy: u32,
    max_y: u32,
    m: &Metrics,
) {
    let g = &state.scrollback.grid;
    for row in 0..VISIBLE_ROWS {
        let y = crate::layout::row_top(row as u32, oy, m.lh);
        if y + m.lh > max_y {
            break;
        }
        let abs = g.abs_of_visible_row(row);
        let (idx, status) = match block_for(state, abs) {
            Some(v) => v,
            None => continue,
        };
        // Alternating block shades derived from the theme background, so the
        // command zebra stays subtle on any profile instead of a fixed dark.
        let tint = if idx % 2 == 0 { elevate(state.bg, 5) } else { elevate(state.bg, 13) };
        fb.fill_rect(ox, y, fb.width.saturating_sub(ox * 2), m.lh, tint);
        let stripe = match status {
            Status::Ok => BLOCK_OK,
            Status::Err => BLOCK_ERR,
            Status::Running => BLOCK_RUN,
        };
        fb.fill_rect(ox.saturating_sub(STRIPE_GAP), y, STRIPE_W, m.lh, stripe);
        if let Some(b) = state.block_at(abs) {
            if b.start_abs == abs {
                draw_meta(fb, b, stripe, y);
            }
        }
    }
}

fn block_for(state: &State, abs: u64) -> Option<(usize, Status)> {
    let b = state.block_at(abs)?;
    let idx = state.blocks.iter().position(|x| x.start_abs == b.start_abs)?;
    Some((idx, b.status))
}
