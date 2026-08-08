// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Where the install-consent modal and its two buttons sit, shared by the
//! click routing and the paint pass so the two can never disagree.

use crate::render::layout::Rect;

const PANEL_W: u32 = 560;
const PANEL_H: u32 = 260;
const BTN_W: u32 = 110;
const BTN_H: u32 = 32;

pub(crate) fn panel_rect(w: u32, h: u32) -> Rect {
    let pw = core::cmp::min(PANEL_W, w);
    let ph = core::cmp::min(PANEL_H, h);
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
    let y = p.y + p.height.saturating_sub(BTN_H + 16);
    let mid = p.x + p.width / 2;
    let x = if approve { mid.saturating_sub(BTN_W + 8) } else { mid + 8 };
    Rect { x, y, width: BTN_W, height: BTN_H }
}

pub(super) fn hit(r: Rect, px: u32, py: u32) -> bool {
    px >= r.x && px < r.x + r.width && py >= r.y && py < r.y + r.height
}
