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

use nonos_app_skeleton::PaintBuffer;
use nonos_policy_proto::Category;

use crate::settings::theme::{TAB_ACTIVE_BG, TAB_ACTIVE_FG, TAB_BG, TAB_FG};

use super::layout::{HEADER_H, TAB_H};

/// The three policy categories and the Wi-Fi tab, in tab order.
const TABS: [(Option<Category>, &[u8]); 4] = [
    (Some(Category::User), b"General"),
    (Some(Category::Identity), b"Network"),
    (Some(Category::Kernel), b"Security"),
    (None, b"Wi-Fi"),
];
/// Width of one of the four tabs in a window `win_w` wide. Shared with the hit
/// test in `on_pointer` so the two cannot disagree about where a tab is.
pub fn tab_width(win_w: u32) -> u32 {
    win_w / 4
}

/// Paint the tab bar. A category tab is active when it matches `active` and the
/// Wi-Fi panel is not showing; the Wi-Fi tab is active when `wifi_active`.
pub fn paint_tabs(fb: &mut PaintBuffer, active: Category, wifi_active: bool, win_w: u32) {
    let tab_w = tab_width(win_w);
    for (i, (tab, label)) in TABS.iter().enumerate() {
        let x = i as u32 * tab_w;
        let is_active = match tab {
            Some(cat) => !wifi_active && *cat == active,
            None => wifi_active,
        };
        let bg = if is_active { TAB_ACTIVE_BG } else { TAB_BG };
        let fg = if is_active { TAB_ACTIVE_FG } else { TAB_FG };
        fb.fill_rect(x, HEADER_H, tab_w, TAB_H, bg);
        fb.text(x + 18, HEADER_H + 8, label, fg);
    }
}
