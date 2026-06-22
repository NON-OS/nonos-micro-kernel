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

use super::constants::{CELL_WIDTH, TEXT_LEFT};
use crate::term::block::{Block, Status};
use crate::term::theme::DIM;

pub(super) fn draw_meta(fb: &mut PaintBuffer, b: &Block, stripe: u32, y: u32) {
    let right = fb.width.saturating_sub(TEXT_LEFT);
    let ts_x = right.saturating_sub(8 * CELL_WIDTH);
    fb.text(ts_x, y + 1, &b.ts, DIM);
    let (dbuf, dlen) = crate::term::dur::fmt_dur(b.dur_ms);
    let dur_x = ts_x.saturating_sub((dlen as u32 + 1) * CELL_WIDTH);
    fb.text(dur_x, y + 1, &dbuf[..dlen], DIM);
    let mark: &[u8] = match b.status {
        Status::Ok => b"\x11",
        Status::Err => b"\x12",
        Status::Running => b"",
    };
    if !mark.is_empty() {
        fb.text(dur_x.saturating_sub(2 * CELL_WIDTH), y + 1, mark, stripe);
    }
}
