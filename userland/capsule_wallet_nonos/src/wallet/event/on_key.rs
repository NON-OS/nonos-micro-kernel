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

use crate::wallet::state::{hydrate, State};

// Who gets a key, in order. Whatever is open and typing into owns the key
// first; only what nothing is typing into reaches the shortcuts. Getting this
// order wrong is how digits became view jumps: one to seven were shortcuts
// before they were figures, so typing an amount navigated away mid-number.
pub fn on_key(state: &mut State, code: u32) -> EventOutcome {
    // The one-time backup screen owns input until the user confirms the
    // phrase is written down. Enter is the only way through.
    if state.backup_active {
        if code == nonos_app_skeleton::KEY_ENTER {
            return super::backup::confirm_backup(state);
        }
        return EventOutcome::Idle;
    }
    // The recovery field owns every key while open, so typed words are never
    // mistaken for view shortcuts.
    if state.recover_active {
        return super::recover::recover_input(state, code);
    }
    // While the import field is open it owns every key, so a typed hex digit is
    // never mistaken for a view shortcut.
    if state.import_active {
        return super::import::import_input(state, code);
    }
    // The field on the current view gets first refusal, and declines anything
    // it cannot use so the shortcuts below still work.
    if let Some(out) = super::field_input::field_input(state, code) {
        return out;
    }
    if code == b'r' as u32 || code == b'R' as u32 {
        hydrate(state);
        return EventOutcome::Repaint;
    }
    super::shortcuts::shortcut(state, code)
}
