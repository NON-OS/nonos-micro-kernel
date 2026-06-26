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

use crate::browser::manifest::WIDTH;
use crate::browser::state::State;

const TOOLBAR_BG: u32 = 0xFF2B_2E37;
const FIELD_BG: u32 = 0xFF15_171C;
const BORDER: u32 = 0xFF3A_3F4B;
const FG: u32 = 0xFFE6_EDF3;
const DIM: u32 = 0xFF8B_98A5;
const ACCENT: u32 = 0xFF8A_B4F8;

pub const TITLEBAR: u32 = 28;
pub const TOOLBAR_H: i32 = 52;
const T: i32 = TITLEBAR as i32;
const BTN_W: i32 = 28;
const BACK_X: i32 = 14;
const FWD_X: i32 = 46;
const RELOAD_X: i32 = 82;
const HOME_X: i32 = 114;
const PILL_L: i32 = 152;
const PILL_R: i32 = WIDTH as i32 - 52;

pub enum Btn {
    Back,
    Forward,
    Reload,
    Home,
    Url,
}

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.fill_rect(0, TITLEBAR, WIDTH, 52, TOOLBAR_BG);
    chevron(fb, BACK_X as u32, true);
    chevron(fb, FWD_X as u32, false);
    reload(fb, RELOAD_X as u32);
    home(fb, HOME_X as u32);
    pill(state, fb);
    hamburger(fb);
    fb.fill_rect(0, TITLEBAR + 51, WIDTH, 1, ACCENT);
}

fn chevron(fb: &mut PaintBuffer, x: u32, _back: bool) {
    fb.fill_rect(x + 4, TITLEBAR + 22, 2, 8, DIM);
    fb.fill_rect(x + 6, TITLEBAR + 20, 2, 2, DIM);
    fb.fill_rect(x + 6, TITLEBAR + 30, 2, 2, DIM);
    fb.fill_rect(x + 8, TITLEBAR + 18, 2, 2, DIM);
    fb.fill_rect(x + 8, TITLEBAR + 32, 2, 2, DIM);
}

fn reload(fb: &mut PaintBuffer, x: u32) {
    fb.fill_rect(x + 2, TITLEBAR + 18, 12, 2, FG);
    fb.fill_rect(x + 2, TITLEBAR + 32, 12, 2, FG);
    fb.fill_rect(x + 2, TITLEBAR + 18, 2, 8, FG);
    fb.fill_rect(x + 12, TITLEBAR + 26, 2, 8, FG);
    fb.fill_rect(x + 10, TITLEBAR + 16, 2, 2, FG);
    fb.fill_rect(x + 12, TITLEBAR + 16, 2, 4, FG);
}

fn home(fb: &mut PaintBuffer, x: u32) {
    fb.fill_rect(x + 6, TITLEBAR + 16, 4, 2, FG);
    fb.fill_rect(x + 4, TITLEBAR + 18, 8, 2, FG);
    fb.fill_rect(x + 2, TITLEBAR + 20, 12, 2, FG);
    fb.fill_rect(x + 4, TITLEBAR + 22, 8, 12, FG);
    fb.fill_rect(x + 7, TITLEBAR + 28, 2, 6, TOOLBAR_BG);
}

fn pill(state: &State, fb: &mut PaintBuffer) {
    let l = PILL_L as u32;
    let w = (PILL_R - PILL_L).max(0) as u32;
    let edge = if state.address_focused { ACCENT } else { BORDER };
    fb.fill_rect(l, TITLEBAR + 10, w, 32, edge);
    fb.fill_rect(l + 1, TITLEBAR + 11, w.saturating_sub(2), 30, FIELD_BG);
    fb.fill_rect(l, TITLEBAR + 10, 2, 2, TOOLBAR_BG);
    fb.fill_rect(l + w - 2, TITLEBAR + 10, 2, 2, TOOLBAR_BG);
    fb.fill_rect(l, TITLEBAR + 40, 2, 2, TOOLBAR_BG);
    fb.fill_rect(l + w - 2, TITLEBAR + 40, 2, 2, TOOLBAR_BG);
    fb.fill_rect(l + 10, TITLEBAR + 22, 8, 8, DIM);
    fb.fill_rect(l + 12, TITLEBAR + 24, 4, 4, FIELD_BG);
    if state.address.is_empty() {
        fb.text(l + 30, TITLEBAR + 18, b"Search or enter address", DIM);
    } else {
        fb.text(l + 30, TITLEBAR + 18, state.address.as_bytes(), FG);
    }
}

fn hamburger(fb: &mut PaintBuffer) {
    let x = WIDTH - 36;
    fb.fill_rect(x, TITLEBAR + 18, 16, 2, DIM);
    fb.fill_rect(x, TITLEBAR + 25, 16, 2, DIM);
    fb.fill_rect(x, TITLEBAR + 32, 16, 2, DIM);
}

pub fn toolbar_button_at(x: i32, y: i32) -> Option<Btn> {
    if y < T || y >= T + TOOLBAR_H {
        return None;
    }
    if hit(x, BACK_X) {
        return Some(Btn::Back);
    }
    if hit(x, FWD_X) {
        return Some(Btn::Forward);
    }
    if hit(x, RELOAD_X) {
        return Some(Btn::Reload);
    }
    if hit(x, HOME_X) {
        return Some(Btn::Home);
    }
    if x >= PILL_L && x < PILL_R {
        return Some(Btn::Url);
    }
    None
}

fn hit(x: i32, base: i32) -> bool {
    x >= base && x < base + BTN_W
}
