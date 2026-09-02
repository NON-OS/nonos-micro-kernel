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

use super::fetch_banner::draw_banner;
use super::fetch_palette::draw_palette;
use super::fetch_uptime::uptime_str;
use crate::term::state::State;
use crate::term::theme::types::Theme;

// Breathing room between the titlebar and the top of the banner. Anchored to
// the body rect so the art starts inside the content area, not butted against
// the chrome above it.
const BANNER_PAD: i32 = 14;
const RULE_W: i32 = 300;
const INFO_PX: f32 = 14.0;
const ROW: i32 = 20;

fn row(fb: &mut PaintBuffer, x: i32, y: i32, label: &str, value: &str, t: &Theme) {
    let _ = fb.text_ttf_mono(x, y, label, t.accent, INFO_PX);
    let _ = fb.text_ttf_mono(x + 96, y, value, t.fg, INFO_PX);
}

pub fn draw_fetch(
    state: &State,
    fb: &mut PaintBuffer,
    x: u32,
    body_y: u32,
    right: u32,
    t: &Theme,
) {
    let ix = x as i32;
    let edge = right as i32;
    let after = draw_banner(fb, ix, body_y as i32 + BANNER_PAD, edge, t);
    let _ = fb.text_ttf(ix, after + 4, "ZeroState Cryptographic OS", t.dim, 14.0);

    let mut y = after + 34;
    let _ = fb.text_ttf_mono(ix, y, "nonos", t.accent, INFO_PX);
    let mut host = [0u8; 40];
    host[0] = b'@';
    let hn = crate::term::identity::hostname();
    let hl = hn.len().min(host.len() - 1);
    host[1..1 + hl].copy_from_slice(&hn[..hl]);
    let at = core::str::from_utf8(&host[..1 + hl]).unwrap_or("@");
    let gap = fb.measure_ttf_mono("nonos", INFO_PX);
    let _ = fb.text_ttf_mono(ix + gap, y, at, t.dim, INFO_PX);
    y += 8;
    fb.fill_rect(ix as u32, (y + 8) as u32, (edge - ix).clamp(0, RULE_W) as u32, 1, t.dim);
    y += 24;

    row(fb, ix, y, "os", "NONOS RAM-resident", t);
    y += ROW;
    let mut kb = [0u8; 48];
    let kn = super::fetch_version::kernel_line(&mut kb);
    row(fb, ix, y, "kernel", core::str::from_utf8(&kb[..kn]).unwrap_or(""), t);
    y += ROW;
    row(fb, ix, y, "shell", "nox   (type 'help')", t);
    y += ROW;
    row(fb, ix, y, "trust", "Ed25519 + ML-DSA-65", t);
    y += ROW;
    row(fb, ix, y, "arch", "x86_64", t);
    y += ROW;

    let now = mk_time_millis();
    let elapsed =
        if now > 0 && now as u64 >= state.start_ms { now as u64 - state.start_ms } else { 0 };
    let mut buf = [0u8; 24];
    let n = uptime_str(elapsed, &mut buf);
    let up = core::str::from_utf8(&buf[..n]).unwrap_or("");
    row(fb, ix, y, "uptime", up, t);
    y += ROW + 6;

    draw_palette(fb, ix as u32, y as u32);
}
