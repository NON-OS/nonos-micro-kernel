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

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind, PaintBuffer};

use super::types::Terminal;
use crate::paint::tab_bar::{feat_hit, nominal_w};
use crate::term::dimensions::{MAX_FONT_SCALE, MIN_FONT_SCALE};
use crate::term::theme::profiles;

const LIGHTS_RESERVE: u32 = 128;

impl Terminal {
    pub(super) fn accessory_w(&self) -> u32 {
        let want = nominal_w(self.tabs.len());
        match self.width.checked_sub(LIGHTS_RESERVE) {
            Some(room) => want.min(room),
            None => want,
        }
    }

    pub(super) fn paint_accessory_inner(&mut self, fb: &mut PaintBuffer) {
        self.acc_w = fb.width;
        crate::paint::draw_tab_bar(&self.tabs, self.active, fb);
    }

    pub(super) fn accessory_event(&mut self, event: InputEvent) -> EventOutcome {
        if event.kind != InputKind::ButtonDown || event.x < 0 || event.y < 0 {
            return EventOutcome::Idle;
        }
        let x = event.x as u32;
        if let Some(f) = feat_hit(x, self.acc_w) {
            match f {
                0 => self.theme = (self.theme + 1) % profiles::COUNT,
                1 => self.font_scale = self.font_scale.saturating_sub(1).max(MIN_FONT_SCALE),
                2 => self.font_scale = (self.font_scale + 1).min(MAX_FONT_SCALE),
                _ => self.cur().scrollback.clear(),
            }
            return EventOutcome::Repaint;
        }
        self.pill_click(x)
    }
}
