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
use crate::decorations::metrics::{BUTTON_GAP, CLOSE_BUTTON_SIZE, TITLEBAR_HEIGHT, TITLEBAR_PADDING};

use super::types::CloseRect;

pub fn maximize_button_rect(width: u32) -> CloseRect {
    let size = CLOSE_BUTTON_SIZE;
    let y = (TITLEBAR_HEIGHT - size) / 2;
    let x = width.saturating_sub(TITLEBAR_PADDING + 2 * size + BUTTON_GAP);
    CloseRect { x, y, size }
}

pub fn minimize_button_rect(width: u32) -> CloseRect {
    let size = CLOSE_BUTTON_SIZE;
    let y = (TITLEBAR_HEIGHT - size) / 2;
    let x = width.saturating_sub(TITLEBAR_PADDING + 3 * size + 2 * BUTTON_GAP);
    CloseRect { x, y, size }
}
