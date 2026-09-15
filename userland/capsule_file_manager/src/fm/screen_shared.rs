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

use super::layout::{CONTENT_X, HEADER_H, PAD_X};
use super::screen_row::empty_state;
use super::state::State;

/// Sharing needs an identity backend the system does not have yet, so this
/// surface says so instead of inventing entries it cannot produce.
pub fn paint_shared(_state: &State, fb: &mut PaintBuffer) {
    let x = CONTENT_X + PAD_X;
    let w = fb.width.saturating_sub(CONTENT_X + PAD_X * 2);
    empty_state(
        fb,
        x,
        HEADER_H + 60,
        w,
        "Nothing shared yet",
        "Sharing needs the identity backend, which is not built.",
    );
}
