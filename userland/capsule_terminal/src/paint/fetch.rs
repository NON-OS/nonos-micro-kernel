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

//! The fresh-terminal splash: the NONOS block banner over a crisp neofetch-style
//! panel of system facts. All text is anti-aliased TrueType.

use nonos_app_skeleton::PaintBuffer;
use nonos_libc::mk_time_millis;

use super::constants::BODY_TOP;
use super::fetch_banner::draw_banner;
use super::fetch_palette::draw_palette;
use super::fetch_uptime::uptime_str;
use crate::term::state::State;
use crate::term::theme::{ACCENT, DIM, FOREGROUND};

const LEFT: i32 = 26;
// Breathing room between the tab strip and the top of the banner. Anchored to
// the body region so the art starts inside the content area, not butted against
// the strip the way it did before.
const BANNER_TOP: i32 = BODY_TOP as i32 + 14;
const INFO_PX: f32 = 14.0;
const ROW: i32 = 20;

fn row(fb: &mut PaintBuffer, x: i32, y: i32, label: &str, value: &str) {
    let _ = fb.text_ttf_mono(x, y, label, ACCENT, INFO_PX);
    let _ = fb.text_ttf_mono(x + 96, y, value, FOREGROUND, INFO_PX);
}

pub fn draw_fetch(state: &State, fb: &mut PaintBuffer) {
    let after = draw_banner(fb, LEFT, BANNER_TOP);
    let _ = fb.text_ttf(LEFT, after + 4, "ZeroState Cryptographic OS", DIM, 14.0);

    let ix = LEFT;
    let mut y = after + 34;
    let _ = fb.text_ttf_mono(ix, y, "nonos", ACCENT, INFO_PX);
    let _ = fb.text_ttf_mono(ix + 54, y, "@capsule", DIM, INFO_PX);
    y += 8;
    fb.fill_rect(ix as u32, (y + 8) as u32, 300, 1, DIM);
    y += 24;

    row(fb, ix, y, "os", "NONOS RAM-resident");
    y += ROW;
    row(fb, ix, y, "kernel", concat!("microkernel ", include_str!("../../../../VERSION")));
    y += ROW;
    row(fb, ix, y, "shell", "nox   (type 'help')");
    y += ROW;
    row(fb, ix, y, "trust", "Ed25519 + ML-DSA-65");
    y += ROW;
    row(fb, ix, y, "arch", "x86_64");
    y += ROW;

    let now = mk_time_millis();
    let elapsed =
        if now > 0 && now as u64 >= state.start_ms { now as u64 - state.start_ms } else { 0 };
    let mut buf = [0u8; 24];
    let n = uptime_str(elapsed, &mut buf);
    let up = core::str::from_utf8(&buf[..n]).unwrap_or("");
    row(fb, ix, y, "uptime", up);
    y += ROW + 6;

    draw_palette(fb, ix as u32, y as u32);
}
