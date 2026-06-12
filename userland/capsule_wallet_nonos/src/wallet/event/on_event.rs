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

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind, KEY_ESC};

use crate::wallet::state::{hydrate, State};

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.kind != InputKind::KeyDown {
        return EventOutcome::Idle;
    }
    if event.code == KEY_ESC {
        return EventOutcome::Close;
    }
    if event.code == b'r' as u32 || event.code == b'R' as u32 {
        hydrate(state);
        return EventOutcome::Repaint;
    }
    match event.code {
        code if code == b'g' as u32 || code == b'G' as u32 => super::generate::generate(state),
        code if code == b'e' as u32 || code == b'E' as u32 => super::sign_eth::sign_eth(state),
        code if code == b'n' as u32 || code == b'N' as u32 => super::sign_nox::sign_nox(state),
        code if code == b'p' as u32 || code == b'P' as u32 => super::sign_both::sign_both(state),
        _ => EventOutcome::Idle,
    }
}
