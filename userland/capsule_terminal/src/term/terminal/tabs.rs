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

use nonos_app_skeleton::{
    EventOutcome, InputEvent, KEY_PAGE_DOWN, KEY_PAGE_UP, MOD_CTRL, MOD_SHIFT,
};

use super::types::Terminal;
use crate::term::state::State;

const MAX_TABS: usize = 9;
const KEY_T: u32 = 0x54;
const KEY_W: u32 = 0x57;

impl Terminal {
    pub(super) fn tab_command(&mut self, event: InputEvent) -> Option<EventOutcome> {
        if !event.is_key_down() || event.flags & MOD_CTRL == 0 {
            return None;
        }
        let shift = event.flags & MOD_SHIFT != 0;
        match event.code {
            KEY_T if shift => self.open_tab(),
            KEY_W if shift => return Some(self.close_tab()),
            KEY_PAGE_DOWN => self.switch(1),
            KEY_PAGE_UP => self.switch(-1),
            c @ 0x31..=0x39 => self.jump((c - 0x31) as usize),
            _ => return None,
        }
        Some(EventOutcome::Repaint)
    }

    fn open_tab(&mut self) {
        if self.tabs.len() < MAX_TABS {
            self.tabs.push(State::new());
            self.active = self.tabs.len() - 1;
        }
    }

    fn close_tab(&mut self) -> EventOutcome {
        if self.tabs.len() <= 1 {
            return EventOutcome::Close;
        }
        self.tabs.remove(self.active);
        if self.active >= self.tabs.len() {
            self.active = self.tabs.len() - 1;
        }
        EventOutcome::Repaint
    }

    fn switch(&mut self, delta: i32) {
        let n = self.tabs.len() as i32;
        self.active = (((self.active as i32 + delta) % n + n) % n) as usize;
    }

    fn jump(&mut self, i: usize) {
        if i < self.tabs.len() {
            self.active = i;
        }
    }
}
