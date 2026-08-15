// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Where the install-consent modal and its two buttons sit, shared by the
//! click routing and the paint pass so the two can never disagree.

use crate::render::layout::Rect;
use crate::render::ui_font;

const PANEL_W_LOGICAL: u32 = 560;
const PANEL_H_LOGICAL: u32 = 330;
const BTN_W_LOGICAL: u32 = 110;
const BTN_H_LOGICAL: u32 = 32;

fn panel_w() -> u32 {
    PANEL_W_LOGICAL * ui_font::scale()
}

fn panel_h() -> u32 {
    PANEL_H_LOGICAL * ui_font::scale()
}

fn btn_w() -> u32 {
    BTN_W_LOGICAL * ui_font::scale()
}

fn btn_h() -> u32 {
    BTN_H_LOGICAL * ui_font::scale()
}

pub(crate) fn panel_rect(w: u32, h: u32) -> Rect {
    let pw = core::cmp::min(panel_w(), w);
    let ph = core::cmp::min(panel_h(), h);
    Rect { x: w.saturating_sub(pw) / 2, y: h.saturating_sub(ph) / 2, width: pw, height: ph }
}

pub(crate) fn approve_rect(w: u32, h: u32) -> Rect {
    button_rect(w, h, true)
}

pub(crate) fn cancel_rect(w: u32, h: u32) -> Rect {
    button_rect(w, h, false)
}

fn button_rect(w: u32, h: u32, approve: bool) -> Rect {
    let p = panel_rect(w, h);
    let s = ui_font::scale();
    let (bw, bh) = (btn_w(), btn_h());
    let y = p.y + p.height.saturating_sub(bh + 16 * s);
    let mid = p.x + p.width / 2;
    let x = if approve { mid.saturating_sub(bw + 8 * s) } else { mid + 8 * s };
    Rect { x, y, width: bw, height: bh }
}

pub(super) fn hit(r: Rect, px: u32, py: u32) -> bool {
    px >= r.x && px < r.x + r.width && py >= r.y && py < r.y + r.height
}
