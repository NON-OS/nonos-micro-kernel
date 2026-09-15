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

extern crate alloc;

use alloc::vec::Vec;

use super::crumbs::crumb_slots;
use super::header_layout::{search_is_field, tool_slots};
use super::header_nav::nav_slots;
use super::header_slots::{HeadHit, Slot, CLEAR_W};
use super::state::State;

/// The clear affordance inside the search field, derived from the field's own
/// slot so the glyph is drawn exactly where the click is tested. It exists only
/// while there is a query to clear and the field is wide enough to hold it.
pub fn clear_slot(state: &State, tools: &[Slot]) -> Option<Slot> {
    if state.query.is_empty() || !search_is_field(state) {
        return None;
    }
    let field = tools.iter().find(|s| s.hit == HeadHit::Search)?;
    let w = CLEAR_W.min(field.w);
    Some(Slot { x: field.x + field.w - w, w, hit: HeadHit::SearchClear })
}

/// Every header control in one list, in hit-test priority order. The clear box
/// sits inside the search field and so must be tested before it; everything
/// else is disjoint, and the order below is the order `head_hit` picks in.
pub fn header_slots(state: &State) -> Vec<Slot> {
    let tools = tool_slots(state);
    let mut out = Vec::new();
    if let Some(clear) = clear_slot(state, &tools) {
        out.push(clear);
    }
    out.extend(nav_slots());
    out.extend(crumb_slots(state));
    out.extend(tools);
    out
}
