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

use crate::browser::manifest::{HEIGHT, WIDTH};
use crate::browser::state::State;

pub struct Shortcut {
    pub label: &'static [u8],
    pub url: &'static str,
    pub badge: &'static [u8],
    pub color: u32,
}

pub const SHORTCUTS: [Shortcut; 4] = [
    Shortcut { label: b"neverssl.com", url: "http://neverssl.com/", badge: b"N", color: 0xFF3F_B950 },
    Shortcut { label: b"example.com", url: "http://example.com/", badge: b"E", color: 0xFF38_8BFD },
    Shortcut { label: b"info.cern.ch", url: "http://info.cern.ch/", badge: b"C", color: 0xFFE5_534B },
    Shortcut { label: b"httpforever.com", url: "http://httpforever.com/", badge: b"H", color: 0xFFA3_71F7 },
];

const PAGE_BG: u32 = 0xFF18_1B20;
const PILL_BG: u32 = 0xFF20_242C;
const FG: u32 = 0xFFE8_EAED;
const ACCENT: u32 = 0xFF8A_B4F8;
const BORDER: u32 = 0xFF3A_3F4B;
const DIM: u32 = 0xFF9A_A0A6;
const WHITE: u32 = 0xFFFF_FFFF;

pub const CONTENT_TOP: u32 = 80;
const PILL_W: u32 = 640;
const PILL_H: u32 = 46;
const PILL_X: u32 = (WIDTH - PILL_W) / 2;
const PILL_Y: u32 = 170;
const COUNT: u32 = 4;
const CELL_W: u32 = 150;
const ROW_X0: u32 = (WIDTH - COUNT * CELL_W) / 2;
const BADGE: u32 = 56;
const BADGE_Y: u32 = 300;

fn center_x(i: u32) -> u32 {
    ROW_X0 + i * CELL_W + CELL_W / 2
}

fn badge_x(i: u32) -> u32 {
    center_x(i).saturating_sub(BADGE / 2)
}

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.fill_rect(0, CONTENT_TOP, WIDTH, HEIGHT.saturating_sub(CONTENT_TOP), PAGE_BG);
    wordmark(fb);
    search_bar(state, fb);
    for i in 0..SHORTCUTS.len() as u32 {
        shortcut(fb, i);
    }
    centered_text(fb, b"Type in the search bar and press Enter", DIM, 470);
}

fn wordmark(fb: &mut PaintBuffer) {
    let advance = fb.glyph_advance().saturating_mul(3);
    let width = advance.saturating_mul(5).min(WIDTH);
    let x0 = (WIDTH - width) / 2;
    let y = 108;
    fb.text_scaled(x0, y, b"N", FG, 3);
    fb.text_scaled(x0 + advance, y, b"\xd8", ACCENT, 3);
    fb.text_scaled(x0 + advance * 2, y, b"NOS", FG, 3);
}

fn search_bar(state: &State, fb: &mut PaintBuffer) {
    let edge = if state.address_focused { ACCENT } else { BORDER };
    fb.fill_rect(PILL_X, PILL_Y, PILL_W, PILL_H, edge);
    fb.fill_rect(PILL_X + 2, PILL_Y + 2, PILL_W - 4, PILL_H - 4, PILL_BG);
    fb.fill_rect(PILL_X, PILL_Y, 3, 3, PAGE_BG);
    fb.fill_rect(PILL_X + PILL_W - 3, PILL_Y, 3, 3, PAGE_BG);
    fb.fill_rect(PILL_X, PILL_Y + PILL_H - 3, 3, 3, PAGE_BG);
    fb.fill_rect(PILL_X + PILL_W - 3, PILL_Y + PILL_H - 3, 3, 3, PAGE_BG);
    globe(fb, PILL_X + 16, PILL_Y + 15);
    let ty = PILL_Y + (PILL_H - 8) / 2;
    let tx = PILL_X + 40;
    if state.address.is_empty() {
        fb.text(tx, ty, b"Search or enter a URL", DIM);
    } else {
        fb.text(tx, ty, state.address.as_bytes(), FG);
    }
}

fn globe(fb: &mut PaintBuffer, x: u32, y: u32) {
    fb.fill_rect(x, y, 16, 16, DIM);
    fb.fill_rect(x, y, 2, 2, PILL_BG);
    fb.fill_rect(x + 14, y, 2, 2, PILL_BG);
    fb.fill_rect(x, y + 14, 2, 2, PILL_BG);
    fb.fill_rect(x + 14, y + 14, 2, 2, PILL_BG);
    fb.fill_rect(x + 7, y, 2, 16, PILL_BG);
    fb.fill_rect(x, y + 7, 16, 2, PILL_BG);
}

fn shortcut(fb: &mut PaintBuffer, i: u32) {
    let s = &SHORTCUTS[i as usize];
    let bx = badge_x(i);
    fb.fill_rect(bx, BADGE_Y, BADGE, BADGE, s.color);
    fb.fill_rect(bx, BADGE_Y, 6, 6, PAGE_BG);
    fb.fill_rect(bx + BADGE - 6, BADGE_Y, 6, 6, PAGE_BG);
    fb.fill_rect(bx, BADGE_Y + BADGE - 6, 6, 6, PAGE_BG);
    fb.fill_rect(bx + BADGE - 6, BADGE_Y + BADGE - 6, 6, 6, PAGE_BG);
    let gw = fb.glyph_advance().saturating_mul(3);
    let glyph_x = center_x(i).saturating_sub(gw / 2);
    fb.text_scaled(glyph_x, BADGE_Y + 16, s.badge, WHITE, 3);
    let lw = (s.label.len() as u32).saturating_mul(fb.glyph_advance());
    let lx = center_x(i).saturating_sub(lw / 2);
    fb.text(lx, 370, s.label, FG);
}

fn centered_text(fb: &mut PaintBuffer, bytes: &[u8], color: u32, y: u32) {
    let width = (bytes.len() as u32 * fb.glyph_advance()).min(WIDTH);
    let x = (WIDTH - width) / 2;
    fb.text(x, y, bytes, color);
}

pub fn shortcut_at(x: i32, y: i32) -> Option<&'static str> {
    if x < 0 || y < 0 {
        return None;
    }
    let (xu, yu) = (x as u32, y as u32);
    for i in 0..SHORTCUTS.len() as u32 {
        let bx = badge_x(i);
        if xu >= bx && xu < bx + BADGE && yu >= BADGE_Y && yu < BADGE_Y + BADGE {
            return Some(SHORTCUTS[i as usize].url);
        }
    }
    None
}

pub fn search_bar_hit(x: i32, y: i32) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    let (xu, yu) = (x as u32, y as u32);
    xu >= PILL_X && xu < PILL_X + PILL_W && yu >= PILL_Y && yu < PILL_Y + PILL_H
}
