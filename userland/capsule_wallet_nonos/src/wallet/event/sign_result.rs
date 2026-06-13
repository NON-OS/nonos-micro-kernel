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

use alloc::vec::Vec;

use nonos_app_skeleton::EventOutcome;

use crate::wallet::state::{record_tx, State};
use crate::wallet::tx_hash::tx_hash;

pub fn sign_result(
    state: &mut State,
    kind: &'static [u8],
    raw: Result<Vec<u8>, i32>,
) -> EventOutcome {
    let Ok(raw) = raw else {
        state.status = b"transaction sign failed";
        return EventOutcome::Repaint;
    };
    let mut hash = [0u8; 32];
    if !tx_hash(&raw, &mut hash) {
        state.status = b"transaction hash failed";
        return EventOutcome::Repaint;
    }
    record_tx(state, kind, &raw, hash);
    state.view = crate::wallet::state::VIEW_PROOF;
    state.status = b"transaction signed";
    EventOutcome::Repaint
}
