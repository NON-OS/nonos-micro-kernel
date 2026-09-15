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

use super::measure_text::{right_text, truncate_to_width, width_of};
use super::theme::{INK2, INK3};

// One label/value line in the info panel, and the advance a column of them
// stacks by, so no caller repeats the arithmetic.
pub const KV_ADV: u32 = 27;
const KV_PX: f32 = 14.0;

/// A label on the left and its value right-aligned on `x + w`, the value cut to
/// whatever the label leaves behind. Returns its own advance.
pub fn kv(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &str, value: &str) -> u32 {
    let _ = fb.text_ttf(x as i32, y as i32, label, INK3, KV_PX);
    let room = w.saturating_sub(width_of(fb, label, KV_PX) + 12);
    let value = truncate_to_width(fb, value, KV_PX, room);
    right_text(fb, x + w, y, value, KV_PX, INK2);
    KV_ADV
}
