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
use super::super::metrics::CARD_GAP;
use super::{licenses_banner, licenses_table, licenses_text};

// Each card's top is the offset the painter places it at and the offset the extent
// is summed from, so the scrollbar cannot drift from the content: adding a card is
// one more const here, consumed by both.
const TABLE_TOP: u32 = licenses_banner::HEIGHT + CARD_GAP;
const TEXT_TOP: u32 = TABLE_TOP + licenses_table::HEIGHT + CARD_GAP;

// The one section whose content is genuinely unbounded: the extent is the banner
// plus the component table plus a row per licence line, and the scroll clamp in
// the frame depends on that count being the same one the painter walks.
pub fn content_h(_rect: &Rect) -> u32 {
    TEXT_TOP + licenses_text::height()
}

pub fn paint(state: &State, fb: &mut PaintBuffer, rect: &Rect) {
    let mut pane = fb.sub(rect.x, rect.y, rect.w, rect.h);
    let y = -(state.scroll as i32);
    licenses_banner::paint(&mut pane, y, rect.w);
    licenses_table::paint(&mut pane, y + TABLE_TOP as i32, rect.w);
    licenses_text::paint(&mut pane, y + TEXT_TOP as i32, rect.w);
}
