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

use nonos_app_skeleton::EventOutcome;

use super::types::Terminal;
use crate::paint::tabstrip::{CLOSE_W, PLUS_W, STRIP_H, STRIP_Y, TAB_W};

impl Terminal {
    pub(super) fn tab_click(&mut self, x: i32, y: i32) -> Option<EventOutcome> {
        if x < 0 || y < STRIP_Y as i32 || y >= (STRIP_Y + STRIP_H) as i32 {
            return None;
        }
        let xu = x as u32;
        let n = self.tabs.len() as u32;
        if xu >= n * TAB_W && xu < n * TAB_W + PLUS_W {
            self.open_tab();
            return Some(EventOutcome::Repaint);
        }
        let i = (xu / TAB_W) as usize;
        if i >= self.tabs.len() {
            return None;
        }
        if xu >= i as u32 * TAB_W + TAB_W - CLOSE_W {
            self.active = i;
            return Some(self.close_tab());
        }
        self.active = i;
        Some(EventOutcome::Repaint)
    }
}
