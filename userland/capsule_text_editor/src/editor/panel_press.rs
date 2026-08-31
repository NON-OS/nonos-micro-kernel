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

//! An open panel takes the whole next press: a Special Character row inserts
//! its text at the caret, anything else just dismisses the panel.

use alloc::vec::Vec;

use nonos_app_skeleton::EventOutcome;

use super::app::Editor;
use super::layout::CHROME_PX;
use super::panel::{panel_title, Panel};
use super::panel_geom::{panel_list, panel_rect};
use super::specials::SPECIALS;
use super::widget::navlist_hit;

impl Editor {
    pub(super) fn open_panel(&mut self, panel: Panel) {
        self.panel = Some(panel);
    }

    pub(super) fn panel_press(&mut self, x: i32, y: i32) -> EventOutcome {
        let Some(panel) = self.panel.take() else {
            return EventOutcome::Repaint;
        };
        if panel != Panel::Special {
            return EventOutcome::Repaint;
        }
        let owned = self.panel_rows(panel);
        let labels: Vec<&str> = owned.iter().map(|s| s.as_str()).collect();
        let rect = panel_rect(self.last_w, self.last_h, panel_title(panel), &labels);
        let row = navlist_hit(panel_list(rect), labels.len(), CHROME_PX, x, y);
        if let Some(text) = row.and_then(|i| SPECIALS.get(i)).map(|(_, text)| *text) {
            let _ = self.doc().insert(text.as_bytes());
        }
        EventOutcome::Repaint
    }
}
