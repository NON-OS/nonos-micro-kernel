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

use crate::wallet::theme::{FG, MUTED, PANEL_2};

pub fn paint_home_security(fb: &mut PaintBuffer, w: u32) {
    if w <= 1080 {
        return;
    }
    fb.fill_rect(368, 690, 360, 72, PANEL_2);
    fb.text(392, 710, b"Keys", MUTED);
    fb.text(392, 738, b"NONOS keyring isolated", FG);
    fb.fill_rect(760, 690, 360, 72, PANEL_2);
    fb.text(784, 710, b"Signing", MUTED);
    fb.text(784, 738, b"EIP-1559 raw tx", FG);
}
