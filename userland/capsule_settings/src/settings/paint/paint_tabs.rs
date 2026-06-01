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

use crate::settings::manifest::WIDTH;
use crate::settings::theme::{TAB_ACTIVE_BG, TAB_ACTIVE_FG, TAB_BG, TAB_FG};

use super::layout::{HEADER_H, TAB_H};

const TABS: [(Category, &[u8]); 3] = [
    (Category::User, b"Display"),
    (Category::Identity, b"Network"),
    (Category::Kernel, b"Security"),
];
const TAB_WIDTH: u32 = WIDTH / 3;

pub fn paint_tabs(fb: &mut PaintBuffer, active: Category) {
    for (i, (tab, label)) in TABS.iter().enumerate() {
        let x = i as u32 * TAB_WIDTH;
        let is_active = *tab == active;
        let bg = if is_active { TAB_ACTIVE_BG } else { TAB_BG };
        let fg = if is_active { TAB_ACTIVE_FG } else { TAB_FG };
        fb.fill_rect(x, HEADER_H, TAB_WIDTH, TAB_H, bg);
        fb.text(x + 18, HEADER_H + 8, label, fg);
    }
}
