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

use nonos_app_skeleton::{EventOutcome, InputEvent};

use super::types::Terminal;
use crate::event::on_event;
use crate::term::dimensions::{MAX_FONT_SCALE, MIN_FONT_SCALE};

impl Terminal {
    pub(super) fn on_event_inner(&mut self, event: InputEvent) -> EventOutcome {
        if let Some(outcome) = self.palette_key(event) {
            return outcome;
        }
        if let Some(outcome) = self.tab_command(event) {
            return outcome;
        }
        if let Some(outcome) = self.rail_click(event) {
            return outcome;
        }
        let outcome = on_event(self.cur(), event);
        self.drain_chrome_req();
        outcome
    }

    fn drain_chrome_req(&mut self) {
        let before = (self.theme, self.font_scale);
        let s = self.cur();
        let theme = s.theme_req.take();
        let zoom = core::mem::take(&mut s.zoom_req);
        if let Some(t) = theme {
            self.theme = t;
        }
        if zoom != 0 {
            let want = self.font_scale as i32 + zoom;
            self.font_scale = want.clamp(MIN_FONT_SCALE as i32, MAX_FONT_SCALE as i32) as u32;
        }
        if (self.theme, self.font_scale) != before {
            self.prefs_dirty = true;
        }
    }
}
