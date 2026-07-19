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

use crate::wallet::theme::{ACCENT, FG, MUTED};

// A designed empty state inside a card: accent rule, title, one-line reason.
pub fn empty_state(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, title: &[u8], reason: &[u8]) {
    super::card::card(fb, x, y, w, 128);
    fb.fill_rect(x + 26, y + 30, 40, 3, ACCENT);
    let t = core::str::from_utf8(title).unwrap_or("");
    let r = core::str::from_utf8(reason).unwrap_or("");
    let _ = fb.text_ttf((x + 26) as i32, (y + 44) as i32, t, FG, 18.0);
    let _ = fb.text_ttf((x + 26) as i32, (y + 84) as i32, r, MUTED, 13.0);
}
