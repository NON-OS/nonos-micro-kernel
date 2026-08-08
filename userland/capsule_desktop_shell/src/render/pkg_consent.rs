// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Paint the package install-consent modal: the kernel-verified slug, its
//! publisher namespace, the trust tier, and every capability bit the manifest
//! asks for, wrapped so none of it falls outside the panel. Button rects come
//! from the handler so paint and hit-test agree.

use alloc::vec::Vec;

use super::consent::{border, button, fill, APPROVE_BG, BORDER, CANCEL_BG, DIM, FG, PANEL};
use crate::render::layout::Rect;
use crate::render::text::draw_overlay_text;
use crate::server::handlers::pkg_consent::{approve_rect, cancel_rect, panel_rect};
use crate::state::Context;

const TITLE: &[u8] = b"Install package?";
const ADVANCE: u32 = 9;
const MARGIN: u32 = 20;
const CAPS_TOP: u32 = 104;
const LINE_H: u32 = 14;
const MAX_LINES: u32 = 6;

pub fn paint_pkg_consent(ctx: &Context) {
    let Some(prompt) = ctx.pending_pkg_install.as_ref() else {
        return;
    };
    let p = panel_rect(ctx.width, ctx.height);
    fill(ctx, p, PANEL);
    border(ctx, p, BORDER);
    let s = &prompt.summary;
    let tier: &[u8] = if s.tier == 2 { b"Publisher-signed" } else { b"NONOS-enrolled" };
    let mut caps = Vec::with_capacity(320);
    caps.extend_from_slice(b"Caps: ");
    super::cap_names::append(s.caps, &mut caps);
    draw_overlay_text(ctx, p.x + MARGIN, p.y + 22, TITLE, FG);
    draw_overlay_text(ctx, p.x + MARGIN, p.y + 44, &s.slug, FG);
    draw_overlay_text(ctx, p.x + MARGIN, p.y + 62, &s.namespace, DIM);
    draw_overlay_text(ctx, p.x + MARGIN, p.y + 80, tier, DIM);
    paint_caps(ctx, p, &caps);
    button(ctx, approve_rect(ctx.width, ctx.height), APPROVE_BG, b"Approve");
    button(ctx, cancel_rect(ctx.width, ctx.height), CANCEL_BG, b"Cancel");
}

fn paint_caps(ctx: &Context, p: Rect, caps: &[u8]) {
    let cols = core::cmp::max(1, (p.width.saturating_sub(MARGIN * 2) / ADVANCE) as usize);
    let mut at = 0usize;
    let mut line = 0u32;
    while at < caps.len() && line < MAX_LINES {
        let end = at + break_at(&caps[at..], cols);
        draw_overlay_text(ctx, p.x + MARGIN, p.y + CAPS_TOP + line * LINE_H, &caps[at..end], DIM);
        at = if caps.get(end) == Some(&b' ') { end + 1 } else { end };
        line += 1;
    }
}

fn break_at(s: &[u8], cols: usize) -> usize {
    if s.len() <= cols {
        return s.len();
    }
    match s[..=cols].iter().rposition(|&b| b == b' ') {
        Some(i) if i > 0 => i,
        _ => cols,
    }
}
