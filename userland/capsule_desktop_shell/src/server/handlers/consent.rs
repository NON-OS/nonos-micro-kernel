// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The consent gate for a runtime-installed third-party app: clicking its
//! Launchpad tile raises a modal instead of launching. Approve runs the app;
//! any other click, Cancel included, dismisses it. Geometry lives here as the
//! single source of truth the paint pass hit-tests against.

use crate::render::layout::Rect;
use crate::server::repaint::repaint;
use crate::state::Context;

const PANEL_W: u32 = 560;
const PANEL_H: u32 = 170;
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

pub(crate) fn click(ctx: &mut Context, px: u32, py: u32) -> bool {
    if ctx.pending_consent.is_none() {
        return false;
    }
    if hit(approve_rect(ctx.width, ctx.height), px, py) {
        if let Some(name) = ctx.pending_consent.take() {
            super::installed_launch::launch(ctx, &name);
        }
    } else {
        ctx.pending_consent = None;
    }
    repaint(ctx);
    true
}

fn hit(r: Rect, px: u32, py: u32) -> bool {
    px >= r.x && px < r.x + r.width && py >= r.y && py < r.y + r.height
}
