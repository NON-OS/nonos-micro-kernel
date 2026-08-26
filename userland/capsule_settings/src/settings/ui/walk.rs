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

use super::metrics::{CARD_GAP, HEAD_H, PANE_PAD_TOP, ROW_H};
use super::walk_metrics::{block_h, head_h, network_rows, row_h};

#[derive(Clone, Copy)]
pub enum Item {
    Head,
    Card,
    CardHead(usize),
    Row(usize, usize),
    Network(usize),
}

/// The one place section geometry is defined. The painter and the hit test both
/// walk it, so a click can never land on pixels a different formula drew.
pub fn walk<F: FnMut(u32, u32, Item)>(state: &State, mut f: F) {
    let mut y = PANE_PAD_TOP;
    f(y, HEAD_H, Item::Head);
    y += HEAD_H;
    for (bi, b) in blocks_for(state.section).iter().enumerate() {
        let bh = block_h(state, b);
        f(y, bh, Item::Card);
        f(y, head_h(b), Item::CardHead(bi));
        let mut ry = y + head_h(b);
        for (ri, r) in b.rows.iter().enumerate() {
            match r {
                Row::Networks => {
                    for i in 0..network_rows(state) {
                        f(ry, ROW_H, Item::Network(i));
                        ry += ROW_H;
                    }
                }
                other => {
                    f(ry, row_h(other), Item::Row(bi, ri));
                    ry += row_h(other);
                }
            }
        }
        y += bh + CARD_GAP;
    }
}

pub fn content_h(state: &State) -> u32 {
    let mut h = PANE_PAD_TOP + HEAD_H;
    for b in blocks_for(state.section) {
        h += block_h(state, b) + CARD_GAP;
    }
    h + PANE_PAD_TOP
}
