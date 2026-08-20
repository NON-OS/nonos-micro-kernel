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

use crate::settings::schema::rows::{Block, Row};
use crate::settings::state::State;

use super::field_note::note_of;
use super::metrics::{CARD_HEAD_H, CARD_HEAD_NOTE_H, ROW_H, ROW_NOTE_H};

pub fn head_h(b: &Block) -> u32 {
    if b.note.is_some() {
        CARD_HEAD_NOTE_H
    } else {
        CARD_HEAD_H
    }
}

pub fn row_h(r: &Row) -> u32 {
    match r {
        Row::Field(f) if note_of(*f).is_some() => ROW_NOTE_H,
        _ => ROW_H,
    }
}

pub fn network_rows(state: &State) -> usize {
    state.wifi_network_count.max(1)
}

pub fn block_h(state: &State, b: &Block) -> u32 {
    let mut h = head_h(b);
    for r in b.rows {
        h += match r {
            Row::Networks => ROW_H * network_rows(state) as u32,
            other => row_h(other),
        };
    }
    h
}
