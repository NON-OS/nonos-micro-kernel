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

use crate::browser::paint::document::VIEW_H;
use crate::browser::state::State;

pub fn scroll_by(state: &mut State, dy: i32) {
    let max = match state.document.as_ref() {
        Some(d) => d.content_h.saturating_sub(VIEW_H),
        None => 0,
    };
    let next = state.scroll as i32 + dy;
    state.scroll = next.clamp(0, max as i32) as u32;
}
