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
use crate::decorations::metrics::{CLOSE_BUTTON_SIZE, TITLEBAR_HEIGHT, TITLEBAR_PADDING};

use super::types::CloseRect;

pub fn close_button_rect(width: u32) -> CloseRect {
    let pad = TITLEBAR_PADDING;
    let size = CLOSE_BUTTON_SIZE;
    let y = (TITLEBAR_HEIGHT - size) / 2;
    let x = width.saturating_sub(pad + size);
    CloseRect { x, y, size }
}
