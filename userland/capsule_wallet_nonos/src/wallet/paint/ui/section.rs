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

use crate::wallet::theme::{ACCENT, FG};

// A section heading: accent tick + TTF title. Sets the type rhythm for a card.
pub fn section(fb: &mut PaintBuffer, x: u32, y: u32, title: &str) {
    fb.fill_rect(x, y + 2, 3, 20, ACCENT());
    let _ = fb.text_ttf((x + 14) as i32, y as i32, title, FG(), 25.0);
}

// The large screen title with a muted eyebrow above it.
pub fn title(fb: &mut PaintBuffer, x: u32, y: u32, eyebrow: &[u8], head: &str) {
    let e = core::str::from_utf8(eyebrow).unwrap_or("");
    let _ = fb.text_ttf(x as i32, y as i32, e, ACCENT(), 10.0);
    let _ = fb.text_ttf(x as i32, (y + 20) as i32, head, FG(), 32.0);
}
