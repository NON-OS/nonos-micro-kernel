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

//! Dispatch input to the screen the window is currently showing, and hold the
//! inline name entry that the Editor screen's router hands the keyboard to.

use nonos_app_skeleton::{EventOutcome, InputEvent};

use super::app::Editor;
use super::home::home_event;
use super::sb_entry::{entry_key, EntryOutcome};
use super::screen::Screen;
use super::settings::settings_event;

impl Editor {
    pub(super) fn handle_event(&mut self, event: InputEvent) -> EventOutcome {
        if self.screen != Screen::Editor {
            if let Some(outcome) = self.activity_event(&event) {
                return outcome;
            }
        }
        match self.screen {
            Screen::Editor => self.handle_editor_event(event),
            Screen::Home => home_event(self, event),
            Screen::Settings => settings_event(self, event),
        }
    }

    pub(super) fn entry_key_event(&mut self, code: u32) -> EventOutcome {
        let Some(entry) = self.entry.as_mut() else {
            return EventOutcome::Idle;
        };
        match entry_key(entry, code, self.owner_pid) {
            EntryOutcome::Pending => {}
            EntryOutcome::Cancelled => self.entry = None,
            EntryOutcome::Committed { renamed } => {
                self.entry = None;
                if let Some((old, new)) = renamed {
                    self.follow_rename(&old, &new);
                }
                self.tree.reload(self.owner_pid);
            }
            EntryOutcome::Failed(e) => {
                self.entry = None;
                self.tree.status = e;
            }
        }
        EventOutcome::Repaint
    }
}
