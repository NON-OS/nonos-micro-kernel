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

use crate::wallet::state::{State, VIEW_NOX, VIEW_SEND, VIEW_SWAP};

/// Offer a key to the field on the current view before anything else sees it.
///
/// A screen with a figure on it owns the keys that figure is made of. Without
/// this the digits one to seven were view shortcuts first, so typing an
/// amount navigated away mid-number: 3000 jumped to Send on its first
/// keystroke and entered 000 on whatever screen it landed on. The same held
/// for an address, where b and d are both hex digits and shortcuts.
///
/// Each handler returns `None` for keys it cannot use, so shortcuts still
/// work for everything that is not part of what is being typed.
pub fn field_input(state: &mut State, code: u32) -> Option<EventOutcome> {
    match state.view {
        VIEW_SEND => super::send_input::send_input(state, code),
        VIEW_SWAP => super::swap_input::swap_input(state, code),
        VIEW_NOX => super::stake_input::stake_input(state, code),
        _ => None,
    }
}
