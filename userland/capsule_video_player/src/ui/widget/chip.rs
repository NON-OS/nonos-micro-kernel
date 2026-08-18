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

use nonos_app_skeleton::paint::PaintBuffer;

use crate::ui::fit::width;
use crate::ui::text::{center_y, BODY_PX};
use crate::ui::paint::rrect;
use crate::ui::theme;

pub const CHIP_H: u32 = 26;
const PAD: u32 = 11;
const GAP: u32 = 8;

pub fn chip_w(label: &str) -> u32 {
    width(label, BODY_PX) + PAD * 2
}

pub fn paint_chip(fb: &mut PaintBuffer, x: u32, y: u32, label: &str, ink: u32, bg: u32) -> u32 {
    let w = chip_w(label);
    rrect::fill_round(fb, x, y, w, CHIP_H, CHIP_H / 2, bg);
    fb.text_ttf((x + PAD) as i32, center_y(y, CHIP_H), label, ink, BODY_PX);
    w + GAP
}

pub fn paint_tag(fb: &mut PaintBuffer, x: u32, y: u32, label: &str) -> u32 {
    paint_chip(fb, x, y, label, theme::TEXT_DIM, theme::PANEL_ALT)
}

pub fn paint_badge(fb: &mut PaintBuffer, x: u32, y: u32, label: &str) -> u32 {
    paint_chip(fb, x, y, label, theme::APP_BG, theme::ACCENT_DIM)
}

pub fn paint_row(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, labels: &[&str]) -> u32 {
    let mut pen = x;
    let mut row = y;
    for label in labels {
        let need = chip_w(label);
        if pen > x && pen + need > x + w {
            pen = x;
            row += CHIP_H + GAP;
        }
        pen += paint_tag(fb, pen, row, label);
    }
    row + CHIP_H
}
