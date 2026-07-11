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

use crate::wallet::theme::PANEL_2;

// A status pill: dim base, tone tick, tone label. Returns the width drawn so
// callers can lay out following content.
pub fn badge(fb: &mut PaintBuffer, x: u32, y: u32, text: &[u8], tone: u32) -> u32 {
    let w = text.len() as u32 * 8 + 30;
    fb.fill_rect(x, y, w, 26, PANEL_2);
    fb.fill_rect(x, y, 3, 26, tone);
    fb.text(x + 12, y + 8, text, tone);
    w
}
