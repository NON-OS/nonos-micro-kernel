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

//! The file-explorer sidebar: an EXPLORER header and one row per visible tree
//! node, with a disclosure chevron for directories, depth indentation, and a
//! highlight on the selected row.

use nonos_app_skeleton::PaintBuffer;

use super::layout::{ACTIVITY_W, CHROME_PX, FOOTER_H, ROW_H, SIDEBAR_W, TABBAR_H, TITLEBAR_H};
use super::shell::pane_y;
use super::theme::{FOLDER, FOREGROUND, HEADER_BG, LINE, MUTED, ROW_SELECT, SIDEBAR_BG};
use super::tree::FileTree;

pub(super) fn paint_sidebar(fb: &mut PaintBuffer, tree: &FileTree, height: u32, entry_open: bool) {
    fb.fill_rect(ACTIVITY_W, 0, SIDEBAR_W, height, SIDEBAR_BG);
    fb.fill_rect(ACTIVITY_W, TITLEBAR_H, SIDEBAR_W, TABBAR_H, HEADER_BG);
    let _ =
        fb.text_ttf((ACTIVITY_W + 14) as i32, (TITLEBAR_H + 10) as i32, "EXPLORER", MUTED, 11.0);
    fb.fill_rect(ACTIVITY_W + SIDEBAR_W - 1, 0, 1, height, LINE);

    // While the name entry is open its bar takes the first row slot.
    let top = pane_y() + if entry_open { ROW_H } else { 0 };
    let avail = height.saturating_sub(top + FOOTER_H);
    let rows = avail / ROW_H;

    if tree.visible.is_empty() {
        let msg = if tree.status.is_empty() { "(empty)" } else { tree.status };
        let _ = fb.text_ttf((ACTIVITY_W + 14) as i32, (top + 6) as i32, msg, MUTED, CHROME_PX);
        return;
    }

    for r in 0..rows {
        let vi = (tree.scroll + r) as usize;
        if vi >= tree.visible.len() {
            break;
        }
        let node = &tree.nodes[tree.visible[vi]];
        let y = top + r * ROW_H;
        if vi == tree.selected {
            fb.fill_rect(ACTIVITY_W, y, SIDEBAR_W, ROW_H, ROW_SELECT);
        }
        let indent = 12 + node.depth as u32 * 12;
        let x = ACTIVITY_W + indent;
        if node.is_dir {
            let expanded = tree.expanded.contains(&node.path);
            let mark = if expanded { "\u{25BE}" } else { "\u{25B8}" };
            let _ = fb.text_ttf(x as i32, (y + 5) as i32, mark, MUTED, 12.0);
        }
        let tx = x + 15;
        let color = if node.is_dir { FOLDER } else { FOREGROUND };
        let name = truncate(&node.name, SIDEBAR_W.saturating_sub(indent + 20));
        let _ = fb.text_ttf(tx as i32, (y + 5) as i32, name, color, CHROME_PX);
    }
}

// The visible-row index at a click y, or None outside the row area. Uses the
// same top offset the painter used, entry bar included, so hits line up.
pub(super) fn sidebar_row_at(
    y: i32,
    tree: &FileTree,
    height: u32,
    entry_open: bool,
) -> Option<usize> {
    let top = pane_y() + if entry_open { ROW_H } else { 0 };
    if y < top as i32 || (y as u32) >= height.saturating_sub(FOOTER_H) {
        return None;
    }
    let r = ((y as u32) - top) / ROW_H;
    let vi = (tree.scroll + r) as usize;
    (vi < tree.visible.len()).then_some(vi)
}

// Cut a name to roughly fit `px_width`, on a char boundary so the slice is
// always valid UTF-8.
fn truncate(name: &str, px_width: u32) -> &str {
    let max_chars = (px_width / 8).max(1) as usize;
    match name.char_indices().nth(max_chars) {
        Some((idx, _)) => &name[..idx],
        None => name,
    }
}
