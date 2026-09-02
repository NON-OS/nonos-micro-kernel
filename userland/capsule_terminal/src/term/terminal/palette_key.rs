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
    EventOutcome, InputEvent, KEY_BACKSPACE, KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_UP, MOD_CTRL,
    MOD_META, MOD_SHIFT,
};

use super::types::Terminal;

const KEY_K: u32 = 0x4B;
const KEY_K_LO: u32 = 0x6B;

impl Terminal {
    /// The palette's key gate, and the first thing the window's event path
    /// runs. While it is open every key-down is answered here, so nothing can
    /// reach the tab bindings, the rail or the shell input line behind it.
    pub(super) fn palette_key(&mut self, event: InputEvent) -> Option<EventOutcome> {
        if !event.is_key_down() {
            return None;
        }
        if !self.palette.open {
            if !opens(event.flags, event.code) {
                return None;
            }
            self.palette.show();
            return Some(EventOutcome::Repaint);
        }
        Some(self.palette_edit(event))
    }

    fn palette_edit(&mut self, event: InputEvent) -> EventOutcome {
        match event.code {
            KEY_ESC => {
                self.palette.hide();
                EventOutcome::Repaint
            }
            KEY_ENTER => self.palette_pick(),
            KEY_UP => self.palette_step(-1),
            KEY_DOWN => self.palette_step(1),
            KEY_BACKSPACE => {
                self.palette.backspace();
                EventOutcome::Repaint
            }
            code if (0x20..=0x7E).contains(&code) => {
                self.palette.push(code as u8);
                EventOutcome::Repaint
            }
            _ => EventOutcome::Idle,
        }
    }
}

/// Ctrl+K, or the command key on a host that lets one through. Shift is
/// excluded so Ctrl+Shift+K still reaches the line editor's kill-to-end.
fn opens(flags: u16, code: u32) -> bool {
    flags & (MOD_CTRL | MOD_META) != 0 && flags & MOD_SHIFT == 0 && matches!(code, KEY_K | KEY_K_LO)
}
