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
use nonos_libc::mk_time_millis;

use super::fetch_palette::draw_palette;
use super::fetch_row::fetch_row;
use super::fetch_uptime::uptime_str;
use crate::term::state::State;
use crate::term::theme::{ACCENT, DIM};

pub fn draw_fetch(state: &State, fb: &mut PaintBuffer) {
    fb.text_scaled(30, 54, b"\xd8", ACCENT, 6);
    fb.text_scaled(22, 112, b"NONOS", ACCENT, 2);
    let ix = 172;
    let mut y = 50;
    fb.text(ix, y, b"nonos", ACCENT);
    fb.text(ix + 45, y, b"@capsule", DIM);
    y += 13;
    fb.fill_rect(ix, y + 4, 168, 1, DIM);
    y += 17;
    fetch_row(fb, ix, y, b"os", b"NONOS RAM-resident");
    y += 16;
    fetch_row(fb, ix, y, b"kernel", b"microkernel 0.8.10");
    y += 16;
    fetch_row(fb, ix, y, b"shell", b"nox");
    y += 16;
    fetch_row(fb, ix, y, b"trust", b"Ed25519 + ML-DSA-65");
    y += 16;
    fetch_row(fb, ix, y, b"arch", b"x86_64");
    y += 16;
    let now = mk_time_millis();
    let elapsed =
        if now > 0 && now as u64 >= state.start_ms { now as u64 - state.start_ms } else { 0 };
    let mut buf = [0u8; 24];
    let n = uptime_str(elapsed, &mut buf);
    fetch_row(fb, ix, y, b"uptime", &buf[..n]);
    y += 22;
    draw_palette(fb, ix, y);
}
