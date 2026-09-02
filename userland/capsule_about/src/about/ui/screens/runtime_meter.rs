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

use crate::about::data::runtime::Runtime;
use crate::about::theme::{ACCENT, TRACK_BG};

use super::super::metrics::{CARD_PAD, METER_H};

// The memory row drawn again as a length. It is filled from the same pair the row
// printed, and a fill narrower than the bar's own end caps is left off entirely
// rather than drawn as a lozenge wider than the value it stands for.
pub fn meter(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, r: &Runtime) {
    if y < 0 || y + METER_H as i32 > fb.height as i32 || r.mem_total_kb == 0 {
        return;
    }
    let used = r.mem_used_kb.min(r.mem_total_kb);
    let fill = ((used * w as u64) / r.mem_total_kb) as u32;
    fb.fill_round(x + CARD_PAD, y as u32, w, METER_H, METER_H / 2, TRACK_BG);
    if fill >= METER_H {
        fb.fill_round(x + CARD_PAD, y as u32, fill, METER_H, METER_H / 2, ACCENT);
    }
}
