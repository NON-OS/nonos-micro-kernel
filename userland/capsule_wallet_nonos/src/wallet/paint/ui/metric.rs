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

use crate::wallet::theme::MUTED;

// A metric tile: card + muted label + emphasized value in `tone`. `unit` is an
// optional trailing suffix (e.g. bps, ETH).
pub fn metric(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &[u8], value: &[u8], unit: &[u8], tone: u32) {
    super::card::card(fb, x, y, w, 80);
    let l = core::str::from_utf8(label).unwrap_or("");
    let v = core::str::from_utf8(value).unwrap_or("");
    let _ = fb.text_ttf((x + 16) as i32, (y + 14) as i32, l, MUTED(), 10.0);
    let pen = fb.text_ttf((x + 16) as i32, (y + 34) as i32, v, tone, 24.0);
    if !unit.is_empty() {
        let u = core::str::from_utf8(unit).unwrap_or("");
        let _ = fb.text_ttf(pen + 6, (y + 46) as i32, u, MUTED(), 13.0);
    }
}
