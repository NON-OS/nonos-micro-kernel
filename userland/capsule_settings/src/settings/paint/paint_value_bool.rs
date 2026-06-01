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

use crate::settings::theme::{VALUE_FG, VALUE_FG_FALSE};

use super::layout::VALUE_LEFT;

pub fn paint_value_bool(fb: &mut PaintBuffer, y: u32, value: Option<bool>) {
    let (text, color): (&[u8], u32) = match value {
        Some(true) => (b"[x] enabled", VALUE_FG),
        Some(false) => (b"[ ] disabled", VALUE_FG_FALSE),
        None => (b"...", VALUE_FG_FALSE),
    };
    fb.text(VALUE_LEFT, y, text, color);
}
