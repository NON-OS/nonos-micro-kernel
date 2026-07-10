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

//! The document tab strip above the code pane. Painting also records each tab's
//! pixel span and its close-box span, so the event router can map a click to a
//! tab or its close button without re-measuring text.

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use super::layout::{CHROME_PX, TABBAR_H, TITLEBAR_H};
use super::state::State;
use super::theme;

#[derive(Clone, Copy)]
pub(super) struct TabSpan {
    pub x0: u32,
    pub x1: u32,
    pub close0: u32,
}

pub(super) fn basename(doc: &State) -> &str {
    let path = core::str::from_utf8(&doc.path[..doc.path_len]).unwrap_or("");
    let name = path.rsplit('/').next().unwrap_or(path);
    if name.is_empty() {
        "untitled"
    } else {
        name
    }
}

pub(super) fn paint_tabs(
    fb: &mut PaintBuffer,
    docs: &[State],
    active: usize,
    pane_x: u32,
    width: u32,
) -> Vec<TabSpan> {
    let th = theme::active();
    fb.fill_rect(pane_x, TITLEBAR_H, width.saturating_sub(pane_x), TABBAR_H, th.tabbar_bg);
    let mut spans = Vec::with_capacity(docs.len());
    let mut x = pane_x;
    for (i, d) in docs.iter().enumerate() {
        let name = basename(d);
        let nw = fb.measure_ttf(name, CHROME_PX).max(0) as u32;
        let modified = !d.undo.is_empty();
        let tw = nw + 46;
        let bg = if i == active { th.tab_active_bg } else { th.tab_inactive_bg };
        fb.fill_rect(x, TITLEBAR_H, tw, TABBAR_H, bg);
        if i == active {
            fb.fill_rect(x, TITLEBAR_H + TABBAR_H - 2, tw, 2, th.accent);
        }
        let color = if i == active { th.title } else { th.muted };
        let _ = fb.text_ttf((x + 12) as i32, (TITLEBAR_H + 9) as i32, name, color, CHROME_PX);
        let close0 = x + tw - 20;
        if modified {
            fb.fill_rect(close0 + 3, TITLEBAR_H + 13, 6, 6, color);
        } else {
            let _ = fb.text_ttf(
                close0 as i32,
                (TITLEBAR_H + 8) as i32,
                "\u{00D7}",
                th.muted,
                CHROME_PX,
            );
        }
        fb.fill_rect(x + tw - 1, TITLEBAR_H, 1, TABBAR_H, th.line);
        spans.push(TabSpan { x0: x, x1: x + tw, close0 });
        x += tw;
        if x >= width {
            break;
        }
    }
    paint_toolbar(fb, width);
    spans
}

// Right-aligned view controls: zoom out, zoom in, cycle background. Clickable so
// they work regardless of keyboard layout. Indices match `toolbar_hit`.
pub(super) const TB_BTN_W: u32 = 34;
pub(super) const TB_BTNS: [&str; 3] = ["A-", "A+", "BG"];

fn paint_toolbar(fb: &mut PaintBuffer, width: u32) {
    let th = theme::active();
    let base = width.saturating_sub(TB_BTNS.len() as u32 * TB_BTN_W);
    for (i, label) in TB_BTNS.iter().enumerate() {
        let bx = base + i as u32 * TB_BTN_W;
        fb.fill_rect(bx + 3, TITLEBAR_H + 5, TB_BTN_W - 5, TABBAR_H - 10, th.tab_inactive_bg);
        let _ = fb.text_ttf((bx + 9) as i32, (TITLEBAR_H + 9) as i32, label, th.accent, CHROME_PX);
    }
}

// Which toolbar button an x falls on (0 zoom out, 1 zoom in, 2 background), or
// None when the click is to the left of them.
pub(super) fn toolbar_hit(x: i32, width: u32) -> Option<usize> {
    if x < 0 {
        return None;
    }
    let x = x as u32;
    let base = width.saturating_sub(TB_BTNS.len() as u32 * TB_BTN_W);
    if x < base {
        return None;
    }
    let i = ((x - base) / TB_BTN_W) as usize;
    (i < TB_BTNS.len()).then_some(i)
}

// Map a click to (tab index, is_close). The close box wins when the click lands
// on it so the tab is closed rather than merely focused.
pub(super) fn tab_hit(spans: &[TabSpan], x: i32) -> Option<(usize, bool)> {
    if x < 0 {
        return None;
    }
    let x = x as u32;
    for (i, s) in spans.iter().enumerate() {
        if x >= s.x0 && x < s.x1 {
            return Some((i, x >= s.close0));
        }
    }
    None
}
