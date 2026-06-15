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

pub fn broadcast(state: &mut State) -> EventOutcome {
    if !state.tx_ready || state.tx_raw.is_empty() {
        state.status = b"no signed tx";
        return EventOutcome::Repaint;
    }
    let Some(hash) = crate::wallet::net::broadcast_raw(&state.tx_raw) else {
        state.status = b"broadcast rejected";
        return EventOutcome::Repaint;
    };
    state.broadcast_hash = hash;
    state.broadcast_ready = true;
    state.receipt_ready = false;
    match crate::wallet::net::poll_receipt(&hash) {
        Some(ok) => {
            state.receipt_ready = true;
            state.receipt_ok = ok;
            state.status = if ok { b"receipt confirmed" } else { b"receipt pending" };
        }
        None => state.status = b"broadcast sent",
    }
    EventOutcome::Repaint
}
