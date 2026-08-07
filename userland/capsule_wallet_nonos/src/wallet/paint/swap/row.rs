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

//! One labelled figure in the terms panel.

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::theme::MUTED;

/// Label left, value right, so a column of figures lines up.
pub fn row(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &str, value: &[u8], tone: u32) {
    let _ = fb.text_ttf((x + 20) as i32, y as i32, label, MUTED(), 13.4);
    let v = core::str::from_utf8(value).unwrap_or("");
    let vw = fb.measure_ttf(v, 14.2).max(0) as u32;
    let _ = fb.text_ttf((x + w - 20 - vw) as i32, (y - 1) as i32, v, tone, 14.2);
}
