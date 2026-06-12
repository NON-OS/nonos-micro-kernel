// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::chips::{chip_width, draw_chip};
use super::layout::splash;
use crate::display::gop::{get_dimensions, is_initialized};

const LABELS: [&[u8]; 3] = [b"SECURE BOOT", b"MEASURED", b"ATTESTED"];

pub fn draw_status_line(secure_boot: bool, measured: bool, attested: bool) {
    if !is_initialized() {
        return;
    }
    let (w, _) = get_dimensions();
    let total: u32 = LABELS.iter().map(|l| chip_width(l)).sum::<u32>() + 20;
    let mut x = (w.saturating_sub(total)) / 2;
    let y = splash().chips_y;
    let states = [secure_boot, measured, attested];
    for (label, on) in LABELS.iter().zip(states) {
        x = draw_chip(x, y, label, on as u8 * 2);
    }
}
