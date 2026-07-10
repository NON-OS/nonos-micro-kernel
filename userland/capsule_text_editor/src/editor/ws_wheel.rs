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

//! Scroll-wheel routing: over the sidebar the file tree scrolls, anywhere else
//! the code pane does. Wheel-up (positive delta) moves toward the start. The
//! caret stays put; only the view moves, and the painter clamps the result.

use nonos_app_skeleton::{EventOutcome, InputEvent};

use super::app::Editor;
use super::layout::ACTIVITY_W;
use super::shell::pane_x;

// Text lines moved per wheel notch, matching what desktops usually do.
const WHEEL_STEP: u32 = 3;

impl Editor {
    pub(super) fn wheel_event(&mut self, event: &InputEvent) -> EventOutcome {
        let over_sidebar = self.sidebar_open
            && event.x >= ACTIVITY_W as i32
            && event.x < pane_x(self.sidebar_open) as i32;
        let steps = event.delta_y.unsigned_abs().min(10) * WHEEL_STEP;
        let up = event.delta_y > 0;

        if over_sidebar {
            let max = self.tree.visible.len().saturating_sub(1) as u32;
            self.tree.scroll = if up {
                self.tree.scroll.saturating_sub(steps)
            } else {
                (self.tree.scroll + steps).min(max)
            };
        } else {
            let d = self.doc();
            d.scroll_line =
                if up { d.scroll_line.saturating_sub(steps) } else { d.scroll_line + steps };
            // The painter clamps scroll_line against the document height, so
            // overshooting past the last line here is harmless.
        }
        EventOutcome::Repaint
    }
}
