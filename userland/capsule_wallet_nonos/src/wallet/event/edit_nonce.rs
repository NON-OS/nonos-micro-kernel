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

use crate::wallet::state::State;

pub fn edit_nonce(state: &mut State, code: u32) -> Option<EventOutcome> {
    let digit = code.checked_sub(b'0' as u32)?;
    if digit < 10 {
        state.send_nonce = state.send_nonce.saturating_mul(10).saturating_add(digit as u64);
        return Some(EventOutcome::Repaint);
    }
    None
}
