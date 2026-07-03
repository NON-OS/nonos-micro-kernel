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

use crate::browser::css::{Computed, Size};

// Clamp a resolved border-box height between min-height and max-height. Length
// values (px, and vh/vw which parse to px) apply directly; percentages need a
// definite parent height, which is not threaded here, so they are ignored.
pub(super) fn resolve_min_max_h(s: &Computed, content_h: i32) -> i32 {
    let mut h = content_h;
    if let Size::Px(mn) = s.min_height {
        h = h.max(mn as i32);
    }
    if let Size::Px(mx) = s.max_height {
        h = h.min(mx as i32);
    }
    h.max(0)
}
