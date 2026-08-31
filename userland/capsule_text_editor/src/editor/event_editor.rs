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

//! Route input through the shell in priority order: the sidebar's name entry
//! takes the keyboard while it is open, an open context menu takes the next
//! click, then the activity bar, the explorer rows (left click opens, right
//! click brings up the menu), the tab strip, and finally the active document.

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};
use nonos_libc::mk_getpid;

use super::app::Editor;
use super::event::on_event;
use super::shell::pane_rect;

impl Editor {
    pub(super) fn handle_editor_event(&mut self, event: InputEvent) -> EventOutcome {
        if self.owner_pid == 0 {
            self.owner_pid = mk_getpid();
        }

        if self.panel.is_some() && event.is_key_down() {
            self.panel = None;
            return EventOutcome::Repaint;
        }

        // The name entry owns the keyboard while it is open.
        if self.entry.is_some() && event.is_key_down() {
            return self.entry_key_event(event.code);
        }

        if event.kind == InputKind::Wheel {
            return self.wheel_event(&event);
        }

        let pointer = matches!(
            event.kind,
            InputKind::ButtonDown | InputKind::PointerAbs | InputKind::ButtonUp
        );
        if pointer {
            if let Some(outcome) = self.pointer_event(&event) {
                return outcome;
            }
        }

        // Code pane and all remaining keyboard input: apply the pane rect,
        // then forward to the document engine.
        let (rx, ry, rw, rh) = pane_rect(self.last_w, self.last_h, self.sidebar_open);
        let d = self.doc();
        d.pane_x = rx;
        d.pane_y = ry;
        d.pane_w = rw;
        d.pane_h = rh;
        on_event(d, event)
    }
}
