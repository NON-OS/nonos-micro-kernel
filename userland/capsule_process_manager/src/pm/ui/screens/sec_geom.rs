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

use super::super::metrics::{CARD_H, PANEL_HEAD_H, PANEL_PAD};

pub const BAND_GAP: u32 = 14;
pub const ALERT_H: u32 = 46;
pub const ALERT_GAP: u32 = 8;
pub const LEVEL_BAR_W: u32 = 3;

// Pane-local, matching table_geom: the painter and the hit test read the same
// source, so a click cannot land on a finding other than the one drawn under it.
pub fn list_y() -> u32 {
    CARD_H + BAND_GAP
}

pub fn row_y(slot: usize) -> u32 {
    list_y() + PANEL_HEAD_H + slot as u32 * (ALERT_H + ALERT_GAP)
}

pub fn visible(pane_h: u32) -> usize {
    let body = pane_h.saturating_sub(list_y() + PANEL_HEAD_H + PANEL_PAD);
    (body / (ALERT_H + ALERT_GAP)) as usize
}

// Index into state.alerts for a pane-local y, or None outside the drawn rows.
// A y inside the gap between two cards counts as the card above it, which is what
// a user aiming at a row expects.
pub fn row_at(pane_h: u32, y: i32, scroll: usize, total: usize) -> Option<usize> {
    let first = row_y(0) as i32;
    if y < first {
        return None;
    }
    let slot = ((y - first) as u32 / (ALERT_H + ALERT_GAP)) as usize;
    if slot >= visible(pane_h) {
        return None;
    }
    let index = scroll + slot;
    if index < total {
        Some(index)
    } else {
        None
    }
}
