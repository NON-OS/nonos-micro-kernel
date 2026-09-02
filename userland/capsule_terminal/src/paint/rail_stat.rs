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

use super::rail_text::{left, lh, right};
use crate::term::theme::types::Theme;

/// One telemetry row: the key on the rail's left edge, the figure measured back
/// from its right. Returns the next row's top so a section reads as a stack.
pub fn stat(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, k: &str, v: &str, t: &Theme) -> i32 {
    left(fb, x, y, k, t.dim);
    right(fb, x + w, y, v, t.fg);
    y + lh() as i32
}
