// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Paint the third-party-app consent modal above the desktop: a centred panel
//! carrying a title, the app name, a permissions note, and Approve / Cancel
//! buttons. Button rects come from the handler so paint and hit-test agree.

use crate::render::fill::fill_rect;
use crate::render::layout::Rect;
use crate::render::text::draw_overlay_text;
use crate::server::handlers::consent::{approve_rect, cancel_rect, panel_rect};
use crate::state::Context;

pub(super) const PANEL: u32 = 0xFF12_1A26;
pub(super) const BORDER: u32 = 0xFF2E_3A4C;
pub(super) const APPROVE_BG: u32 = 0xFF1E_7A3C;
pub(super) const CANCEL_BG: u32 = 0xFF3A_2430;
pub(super) const FG: u32 = 0xFFDF_EAF7;
pub(super) const DIM: u32 = 0xFFAF_BED2;
const GLYPH_ADV: u32 = 8;

const TITLE: &[u8] = b"Launch third-party app?";
const NOTE: &[u8] = b"Publisher-signed. The system grants only its manifest permissions.";

pub fn paint_consent(ctx: &Context) {
    let Some(name) = ctx.pending_consent.as_deref() else {
        return;
    };
    let p = panel_rect(ctx.width, ctx.height);
    fill(ctx, p, PANEL);
    border(ctx, p, BORDER);
    draw_overlay_text(ctx, p.x + 20, p.y + 22, TITLE, FG);
    draw_overlay_text(ctx, p.x + 20, p.y + 52, name, FG);
    draw_overlay_text(ctx, p.x + 20, p.y + 76, NOTE, DIM);
    button(ctx, approve_rect(ctx.width, ctx.height), APPROVE_BG, b"Approve");
    button(ctx, cancel_rect(ctx.width, ctx.height), CANCEL_BG, b"Cancel");
}

pub(super) fn button(ctx: &Context, r: Rect, bg: u32, label: &[u8]) {
    fill(ctx, r, bg);
    border(ctx, r, BORDER);
    let tx = r.x + r.width.saturating_sub(label.len() as u32 * GLYPH_ADV) / 2;
    draw_overlay_text(ctx, tx, r.y + r.height / 2 - 4, label, FG);
}

pub(super) fn fill(ctx: &Context, r: Rect, argb: u32) {
    fill_rect(ctx.backing_va, ctx.stride, ctx.width, ctx.height, r.x, r.y, r.width, r.height, argb);
}

pub(super) fn border(ctx: &Context, r: Rect, argb: u32) {
    let (va, st, w, h) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    fill_rect(va, st, w, h, r.x, r.y, r.width, 1, argb);
    fill_rect(va, st, w, h, r.x, r.y + r.height - 1, r.width, 1, argb);
    fill_rect(va, st, w, h, r.x, r.y, 1, r.height, argb);
    fill_rect(va, st, w, h, r.x + r.width - 1, r.y, 1, r.height, argb);
}
