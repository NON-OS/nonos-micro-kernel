// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Paint the third-party-app consent modal above the desktop: a centred panel
//! carrying a title, the app name, a permissions note, and Approve / Cancel
//! buttons. Button rects come from the handler so paint and hit-test agree.
//! Every width is measured from the face, so nothing the user must read before
//! granting a capability can silently run past the panel edge.

use crate::render::fill::fill_rect;
use crate::render::layout::Rect;
use crate::render::measure_aa::{measure_aa, truncate_to_width};
use crate::render::text_aa::text_aa;
use crate::render::ui_font;
use crate::render::ui_font::{line_h, top_y_centered, valid_str, TITLE_PX, UI_PX};
use crate::server::handlers::consent::{approve_rect, cancel_rect, panel_rect};
use crate::state::Context;

pub(super) const PANEL: u32 = 0xFF12_1A26;
pub(super) const BORDER: u32 = 0xFF2E_3A4C;
pub(super) const APPROVE_BG: u32 = 0xFF1E_7A3C;
pub(super) const CANCEL_BG: u32 = 0xFF3A_2430;
pub(super) const FG: u32 = 0xFFDF_EAF7;
pub(super) const DIM: u32 = 0xFFAF_BED2;
const MARGIN_LOGICAL: u32 = 20;

const ELLIPSIS: &str = "...";
const TITLE: &str = "Launch third-party app?";
const NOTE: &str = "Publisher-signed. The system grants only its manifest permissions.";

pub(super) fn margin() -> u32 {
    MARGIN_LOGICAL * ui_font::scale()
}

pub fn paint_consent(ctx: &Context) {
    let Some(name) = ctx.pending_consent.as_deref() else {
        return;
    };
    let p = panel_rect(ctx.width, ctx.height);
    fill(ctx, p, PANEL);
    border(ctx, p, BORDER);
    let x = p.x + margin();
    let max_w = p.width.saturating_sub(margin() * 2);
    let mut y = p.y + margin();
    line(ctx, x, y, TITLE, FG, TITLE_PX, max_w);
    y += line_h(TITLE_PX);
    line(ctx, x, y, valid_str(name), FG, UI_PX, max_w);
    y += line_h(UI_PX);
    note(ctx, x, y, max_w);
    button(ctx, approve_rect(ctx.width, ctx.height), APPROVE_BG, "Approve");
    button(ctx, cancel_rect(ctx.width, ctx.height), CANCEL_BG, "Cancel");
}

fn note(ctx: &Context, x: u32, top_y: u32, max_w: u32) {
    let mut rest = NOTE;
    let mut y = top_y;
    while !rest.is_empty() {
        let end = wrap_at(rest, UI_PX, max_w);
        line(ctx, x, y, &rest[..end], DIM, UI_PX, max_w);
        rest = rest[end..].trim_start();
        y += line_h(UI_PX);
    }
}

pub(super) fn line(ctx: &Context, x: u32, top_y: u32, text: &str, argb: u32, px: f32, max_w: u32) {
    let fitted = truncate_to_width(text, px, max_w);
    if fitted.len() == text.len() {
        text_aa(ctx, x, top_y, text, argb, px);
        return;
    }
    let room = max_w.saturating_sub(measure_aa(ELLIPSIS, px));
    let cut = truncate_to_width(text, px, room);
    let advance = text_aa(ctx, x, top_y, cut, argb, px);
    text_aa(ctx, x + advance, top_y, ELLIPSIS, argb, px);
}

pub(super) fn wrap_at(text: &str, px: f32, max_w: u32) -> usize {
    let fitted = truncate_to_width(text, px, max_w);
    if fitted.len() == text.len() {
        return text.len();
    }
    match fitted.rfind(' ') {
        Some(i) if i > 0 => i,
        _ if !fitted.is_empty() => fitted.len(),
        _ => text.char_indices().nth(1).map_or(text.len(), |(i, _)| i),
    }
}

pub(super) fn button(ctx: &Context, r: Rect, bg: u32, label: &str) {
    fill(ctx, r, bg);
    border(ctx, r, BORDER);
    let inner = r.width.saturating_sub(8 * ui_font::scale());
    let fitted = truncate_to_width(label, UI_PX, inner);
    let tx = r.x + r.width.saturating_sub(measure_aa(fitted, UI_PX)) / 2;
    text_aa(ctx, tx, top_y_centered(r.y, r.height, UI_PX), fitted, FG, UI_PX);
}

pub(super) fn fill(ctx: &Context, r: Rect, argb: u32) {
    fill_rect(ctx.backing_va, ctx.stride, ctx.width, ctx.height, r.x, r.y, r.width, r.height, argb);
}

pub(super) fn border(ctx: &Context, r: Rect, argb: u32) {
    let (va, st, w, h) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    let s = ui_font::scale();
    fill_rect(va, st, w, h, r.x, r.y, r.width, s, argb);
    fill_rect(va, st, w, h, r.x, r.y + r.height.saturating_sub(s), r.width, s, argb);
    fill_rect(va, st, w, h, r.x, r.y, s, r.height, argb);
    fill_rect(va, st, w, h, r.x + r.width.saturating_sub(s), r.y, s, r.height, argb);
}
