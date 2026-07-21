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

use super::{paint_icons as ic, ui};
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, BG, DIM, FG, LINE, MUTED, PANEL_2, SEL};

pub const CMDS: [&str; 8] = [
    "Go to Home",
    "Go to Receive",
    "Go to Send",
    "Go to Proof",
    "Go to Shielded",
    "Go to NOX",
    "Toggle theme",
    "Lock wallet",
];

pub fn paint_panels(state: &State, fb: &mut PaintBuffer) {
    if state.locked {
        return lock_screen(fb);
    }
    match state.panel {
        1 => command(fb),
        2 => messages(fb),
        3 => account(fb, state),
        _ => {}
    }
}

fn lock_screen(fb: &mut PaintBuffer) {
    fb.fill_rect(0, 0, fb.width, fb.height, BG());
    let cx = fb.width / 2;
    ic::lock(fb, cx - 17, 302, ACCENT());
    center(fb, cx, 352, "Wallet locked", FG(), 22.0);
    center(fb, cx, 386, "Click anywhere to unlock", MUTED(), 14.0);
}

fn command(fb: &mut PaintBuffer) {
    let x = 410u32;
    ui::card(fb, x, 150, 460, 396);
    let _ = fb.text_ttf((x + 20) as i32, 166, "COMMAND PALETTE", DIM(), 10.5);
    ui::bordered(fb, x + 20, 188, 420, 34, PANEL_2(), LINE());
    let _ = fb.text_ttf((x + 32) as i32, 197, "Type a command\u{2026}", MUTED(), 14.0);
    for (i, cmd) in CMDS.iter().enumerate() {
        let y = 240 + i as u32 * 36;
        let _ = fb.text_ttf((x + 24) as i32, (y + 8) as i32, cmd, FG(), 14.0);
    }
}

fn messages(fb: &mut PaintBuffer) {
    let x = 860u32;
    ui::card(fb, x, 82, 300, 120);
    let _ = fb.text_ttf((x + 18) as i32, 98, "NOTIFICATIONS", DIM(), 10.5);
    // No event feed is wired, so the panel does not fabricate one.
    let _ = fb.text_ttf((x + 18) as i32, 132, "No notifications", MUTED(), 13.0);
}

fn account(fb: &mut PaintBuffer, state: &State) {
    let x = 994u32;
    ui::card(fb, x, 82, 260, 96);
    let _ = fb.text_ttf((x + 18) as i32, 98, "ACCOUNT", DIM(), 10.5);
    fb.fill_rect(x + 8, 118, 244, 34, SEL());
    fb.fill_rect(x + 8, 118, 3, 34, ACCENT());
    let _ = fb.text_ttf((x + 20) as i32, 122, "Main", FG(), 14.0);
    if state.address_ready {
        let mut sa = [0u8; 13];
        super::super::hex::short_addr(&state.address, &mut sa);
        let s = core::str::from_utf8(&sa).unwrap_or("");
        let _ = fb.text_ttf_mono((x + 20) as i32, 138, s, MUTED(), 11.5);
    } else {
        let _ = fb.text_ttf((x + 20) as i32, 138, "not generated", MUTED(), 11.5);
    }
}

fn center(fb: &mut PaintBuffer, cx: u32, y: u32, s: &str, color: u32, px: f32) {
    let w = fb.measure_ttf(s, px).max(0) as u32;
    let _ = fb.text_ttf((cx - w / 2) as i32, y as i32, s, color, px);
}
