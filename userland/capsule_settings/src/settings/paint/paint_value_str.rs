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

pub fn paint_value_str(fb: &mut PaintBuffer, y: u32, value: Option<&[u8]>, editing: bool) {
    let prefix: &[u8] = if editing { b">" } else { b" " };
    fb.text(VALUE_LEFT - 12, y, prefix, VALUE_FG);
    match value {
        Some(bytes) if !bytes.is_empty() => fb.text(VALUE_LEFT, y, bytes, VALUE_FG),
        Some(_) => fb.text(VALUE_LEFT, y, b"(empty)", VALUE_FG_FALSE),
        None => fb.text(VALUE_LEFT, y, b"...", VALUE_FG_FALSE),
    }
    if editing {
        fb.text(VALUE_LEFT + 280, y, b"_", VALUE_FG);
    }
}
