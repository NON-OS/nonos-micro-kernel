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

use crate::pm::format::{mem_human, pct_1dp, state_label, u32_decimal, uptime_human};
use crate::pm::state::Row;
use crate::pm::theme::{FOREGROUND, MUTED, WARNING};

use super::chrome::Rect;
use super::metrics::{BODY_PX, CELL_PAD_X, NUM_PX, RISK_SLOT_H, ROW_H};
use super::table_geom::{self, Col};
use super::tint::state_tint;
use super::{risk_strip, text};

// Every numeric cell is placed from its right edge so the digits form a column
// the eye can compare down. Name belongs to the row painter, which owns the
// protected mark that shifts the text.
pub fn paint(fb: &mut PaintBuffer, r: &Rect, cols: &[Col], row: &Row, col: Col, y: u32) {
    let x = r.x + table_geom::col_x(cols, r.w, col);
    let right = x + table_geom::col_w(cols, r.w, col).saturating_sub(CELL_PAD_X);
    let top = text::centred_top(y, ROW_H, BODY_PX);
    let mut buf = [0u8; 24];
    let (n, tint) = match col {
        Col::Pid => (u32_decimal(row.pid, &mut buf), MUTED),
        Col::Cpu => (pct_1dp(row.cpu_pct, &mut buf), FOREGROUND),
        Col::Mem => (mem_human(row.mem_kb, &mut buf), FOREGROUND),
        Col::Uptime => (uptime_human(row.uptime_ms / 1000, &mut buf), WARNING),
        Col::State => {
            text::left(fb, x, top, state_label(row.state), state_tint(row.state), BODY_PX);
            return;
        }
        Col::Auth => {
            risk_strip::paint(fb, x, y + (ROW_H - RISK_SLOT_H) / 2, row.caps);
            return;
        }
        Col::Name => return,
    };
    text::mono_right(fb, right, top, &buf[..n], tint, NUM_PX);
}
