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

use super::format_u32::format_u32;
use super::ui;
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, MUTED, NEUTRAL_800, OK, PANEL};

pub fn paint_statusbar(state: &State, fb: &mut PaintBuffer) {
    let y = fb.height.saturating_sub(54);
    fb.fill_rect(304, y, fb.width.saturating_sub(304), 54, PANEL);
    fb.fill_rect(304, y, fb.width.saturating_sub(304), 1, NEUTRAL_800);
    let _ = fb.text_ttf(336, (y + 18) as i32, "Status", MUTED, 12.0);
    ui::badge(fb, 400, y + 15, state.status, OK);

    // Live input readout. `n` climbs on every discrete key/button event that
    // reaches us: if it advances when you click, pointer delivery works and the
    // bug is in hit-testing; if it only moves on keypress, clicks are not being
    // delivered to the capsule at all. `btn`/`key` shows the last kind.
    let mut buf = [0u8; 48];
    let mut n = 0;
    n += write(&mut buf, n, b"in ");
    let mut num = [0u8; 10];
    let d = format_u32(state.in_count, &mut num);
    n += write(&mut buf, n, &num[..d]);
    n += write(&mut buf, n, if state.in_kind == 5 { b"  btn " } else { b"  key " });
    let d = format_u32(state.in_x.max(0) as u32, &mut num);
    n += write(&mut buf, n, b"x");
    n += write(&mut buf, n, &num[..d]);
    let d = format_u32(state.in_y.max(0) as u32, &mut num);
    n += write(&mut buf, n, b" y");
    n += write(&mut buf, n, &num[..d]);
    let tone = if state.in_kind == 5 { ACCENT } else { MUTED };
    let rx = fb.width.saturating_sub(360);
    let s = core::str::from_utf8(&buf[..n]).unwrap_or("");
    let _ = fb.text_ttf(rx as i32, (y + 18) as i32, s, tone, 12.0);
}

fn write(out: &mut [u8], at: usize, src: &[u8]) -> usize {
    let n = core::cmp::min(out.len().saturating_sub(at), src.len());
    out[at..at + n].copy_from_slice(&src[..n]);
    n
}
