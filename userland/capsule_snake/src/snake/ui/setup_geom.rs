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

use nonos_toolkit::font::ttf::line_height;

use super::metrics::{BTN_H, BTN_W, CHIP_H, GAP, GAP_TIGHT, GAP_WIDE, PX_HEAD, ROW_H};
use super::rect::{self, Rect};
use super::setup_geom_rows::TOGGLES;

pub const SECTIONS: usize = 3;

pub fn head_h() -> u32 {
    line_height(PX_HEAD).max(1) as u32
}

pub fn band_h(section: usize) -> u32 {
    match section {
        2 => ROW_H * TOGGLES as u32,
        _ => CHIP_H,
    }
}

fn section_h(section: usize) -> u32 {
    head_h() + GAP_TIGHT + band_h(section)
}

fn panel_h() -> u32 {
    let sections: u32 = (0..SECTIONS).map(section_h).sum();
    sections + GAP_WIDE * SECTIONS as u32 + BTN_H
}

pub fn panel(w: u32, h: u32) -> Rect {
    rect::centred(rect::content(w, h), BTN_W * 2 + GAP, panel_h())
}

fn section_top(w: u32, h: u32, section: usize) -> u32 {
    let prior: u32 = (0..section.min(SECTIONS)).map(|s| section_h(s) + GAP_WIDE).sum();
    panel(w, h).1 + prior
}

pub fn head(w: u32, h: u32, section: usize) -> Rect {
    let p = panel(w, h);
    (p.0, section_top(w, h, section), p.2, head_h())
}

pub fn band(w: u32, h: u32, section: usize) -> Rect {
    let p = panel(w, h);
    let top = section_top(w, h, section) + head_h() + GAP_TIGHT;
    (p.0, top, p.2, band_h(section))
}

pub fn start(w: u32, h: u32) -> Rect {
    let p = panel(w, h);
    let x = p.0 + p.2.saturating_sub(BTN_W) / 2;
    (x, p.1 + p.3.saturating_sub(BTN_H), BTN_W, BTN_H)
}

pub fn start_at(w: u32, h: u32, x: i32, y: i32) -> bool {
    rect::hit(start(w, h), x, y)
}
