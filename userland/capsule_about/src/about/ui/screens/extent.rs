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

use crate::about::section::Section;
use crate::about::state::State;

use super::super::chrome::Rect;
use super::{display, licenses, overview, system, trust};

// How tall the active section's content is. The frame asks before it paints so
// the scroll offset is already clamped when the painter reads it; a section
// that fits its pane reports the pane height and is therefore unscrollable.
pub fn content_h(state: &State, rect: &Rect) -> u32 {
    let natural = match state.section {
        Section::Overview => overview::content_h(rect),
        Section::System => system::content_h(rect),
        Section::Trust => trust::content_h(rect),
        Section::Display => display::content_h(rect),
        Section::Licenses => licenses::content_h(rect),
    };
    natural.max(rect.h)
}
