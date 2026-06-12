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

use crate::wallet::ipc::sign_nox_approve;
use crate::wallet::state::State;

pub fn sign_nox(state: &mut State) -> EventOutcome {
    if state.wallet_id == 0 {
        state.status = b"generate wallet first";
        return EventOutcome::Repaint;
    }
    let raw = sign_nox_approve(state.keyring_port, state.owner_pid, state.wallet_id);
    super::sign_result::sign_result(state, b"NOX", raw)
}
