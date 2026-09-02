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
use crate::paint::tab_bar::tabs_avail;
use crate::paint::tab_pill::{close_rect, pill_rect, plus_rect};

impl Terminal {
    pub(super) fn pill_click(&mut self, x: u32) -> EventOutcome {
        let avail = tabs_avail(self.acc_w);
        let plus = plus_rect(self.tabs.len(), avail);
        if x >= plus.x && x < plus.x + plus.w {
            self.open_tab();
            return EventOutcome::Repaint;
        }
        for i in 0..self.tabs.len() {
            let r = pill_rect(i, avail);
            if r.w == 0 || x < r.x || x >= r.x + r.w {
                continue;
            }
            self.active = i;
            if x >= close_rect(r).x && self.tabs.len() > 1 {
                return self.close_tab();
            }
            return EventOutcome::Repaint;
        }
        EventOutcome::Idle
    }
}
