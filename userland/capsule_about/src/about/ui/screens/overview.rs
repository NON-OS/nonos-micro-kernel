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

use crate::about::state::State;

use super::super::chrome::Rect;
use super::super::gauge;
use super::super::metrics::{CARD_GAP, HERO_H};
use super::{overview_cards, overview_hero, overview_tiles};

const TILES_Y: u32 = HERO_H + CARD_GAP;
const CARDS_Y: u32 = TILES_Y + gauge::HEIGHT + CARD_GAP;

// The stack is fixed, so its extent is a constant rather than a measurement: hero,
// gauge row, then the two half-width cards. Nothing here wraps, which is why this
// screen reports a height the pane can hold and never scrolls.
pub fn content_h(_rect: &Rect) -> u32 {
    CARDS_Y + overview_cards::HEIGHT
}

pub fn paint(state: &State, fb: &mut PaintBuffer, rect: &Rect) {
    let mut pane = fb.sub(rect.x, rect.y, rect.w, rect.h);
    let y = -(state.scroll as i32);
    overview_hero::paint(&mut pane, y, rect.w);
    overview_tiles::paint(&mut pane, y + TILES_Y as i32, rect.w);
    overview_cards::paint(&mut pane, y + CARDS_Y as i32, rect.w);
}
