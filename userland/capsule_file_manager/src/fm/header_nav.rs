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

use alloc::{vec, vec::Vec};

use super::header_slots::{HeadHit, Slot, ICON_BTN};
use super::layout::{CONTENT_X, PAD_X};
use super::state::State;

/// Left edge of the header's leading navigation group.
pub fn nav_x() -> u32 {
    CONTENT_X + PAD_X
}

/// Right edge of that group. The breadcrumb starts from here, so the two never
/// have to agree on a width twice.
pub fn nav_right() -> u32 {
    nav_x() + ICON_BTN * 2
}

/// The two navigation buttons, sharing one plate. The painter draws these and
/// `head_hit` tests them, so neither can drift from the other.
pub fn nav_slots() -> Vec<Slot> {
    vec![
        Slot { x: nav_x(), w: ICON_BTN, hit: HeadHit::NavBack },
        Slot { x: nav_x() + ICON_BTN, w: ICON_BTN, hit: HeadHit::NavFwd },
    ]
}

/// Whether the backward control has anywhere to go. The capsule keeps no visit
/// stack, so backward means the parent directory and is live everywhere but the
/// root.
pub fn can_back(state: &State) -> bool {
    state.prefix != "/"
}

/// Whether the forward control has anywhere to go. Nothing in the capsule
/// records a directory the user has left, so this is always false and the
/// control is always drawn dimmed rather than silently doing nothing.
pub fn can_fwd(_state: &State) -> bool {
    false
}
