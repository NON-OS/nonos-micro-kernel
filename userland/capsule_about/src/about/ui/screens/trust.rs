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

use super::super::card;
use super::super::chrome::Rect;
use super::super::metrics::CARD_GAP;
use super::{trust_caps, trust_caps_list, trust_chain};

// Both cards wrap against the pane width, so the extent is measured rather than
// declared: the evidence paragraph and the denied pills each take as many rows as
// the face gives them, and the scroll clamp is only right if it asks them.
pub fn content_h(rect: &Rect) -> u32 {
    let inner = card::inner(rect.w);
    trust_chain::height(inner) + CARD_GAP + trust_caps_list::height(inner)
}

pub fn paint(state: &State, fb: &mut PaintBuffer, rect: &Rect) {
    let mut pane = fb.sub(rect.x, rect.y, rect.w, rect.h);
    let y = -(state.scroll as i32);
    let inner = card::inner(rect.w);
    trust_chain::paint(&mut pane, y, rect.w);
    let caps_y = y + (trust_chain::height(inner) + CARD_GAP) as i32;
    trust_caps::paint(&mut pane, caps_y, rect.w);
}
