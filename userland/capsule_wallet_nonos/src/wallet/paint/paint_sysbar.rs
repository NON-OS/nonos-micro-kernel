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

use crate::wallet::theme::{ACCENT, DIM, MUTED, SYSBAR, LINE, WIDTH};

pub fn paint_sysbar(fb: &mut PaintBuffer) {
    fb.fill_rect(0, 0, WIDTH, 34, SYSBAR);
    fb.fill_rect(0, 33, WIDTH, 1, LINE);
    fb.fill_rect(16, 12, 11, 11, ACCENT);
    let _ = fb.text_ttf_mono(34, 9, "CAPSULE_WALLET_NONOS  CPL=3", MUTED, 11.0);
    let right = "12:39  fps 60  1280x800";
    let w = fb.measure_ttf_mono(right, 11.0).max(0) as u32;
    let _ = fb.text_ttf_mono((WIDTH - 16 - w) as i32, 9, right, DIM, 11.0);
}
