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

use super::on_pointer::hit;
use crate::wallet::state::{
    State, SEND_FIELD_AMOUNT, SEND_FIELD_TO, VIEW_NOX, VIEW_RECEIVE, VIEW_SEND,
};

// Home quick actions route to their screens.
pub(super) fn home(state: &mut State, x: u32, y: u32) -> EventOutcome {
    let v = if hit(x, y, 226, 386, 245, 82) {
        VIEW_SEND
    } else if hit(x, y, 487, 386, 245, 82) {
        VIEW_RECEIVE
    } else if hit(x, y, 748, 386, 245, 82) || hit(x, y, 1009, 386, 245, 82) {
        VIEW_NOX
    } else {
        return EventOutcome::Idle;
    };
    state.view = v;
    EventOutcome::Repaint
}

pub(super) fn receive(state: &mut State, x: u32, y: u32) -> EventOutcome {
    use crate::wallet::paint::{GEN_BTN_H, GEN_BTN_W, GEN_BTN_X, GEN_BTN_Y};
    if hit(x, y, GEN_BTN_X, GEN_BTN_Y, GEN_BTN_W, GEN_BTN_H) || hit(x, y, 1104, 320, 130, 42) {
        return if state.address_ready {
            super::probe_tick::probe_kick(state)
        } else {
            super::generate::generate(state)
        };
    }
    EventOutcome::Idle
}

pub(super) fn send(state: &mut State, x: u32, y: u32) -> EventOutcome {
    if hit(x, y, 246, 182, 600, 40) {
        state.send_focus = SEND_FIELD_TO;
    } else if hit(x, y, 246, 342, 508, 40) {
        state.send_focus = SEND_FIELD_AMOUNT;
    } else if hit(x, y, 750, 318, 44, 24) {
        state.usd_mode = false;
    } else if hit(x, y, 794, 318, 44, 24) {
        state.usd_mode = true;
    } else if hit(x, y, 246, 440, 192, 58) {
        state.fee_tier = 0;
    } else if hit(x, y, 450, 440, 192, 58) {
        state.fee_tier = 1;
    } else if hit(x, y, 654, 440, 192, 58) {
        state.fee_tier = 2;
    } else if hit(x, y, 246, 636, 150, 42) {
        return super::sign_eth::sign_eth(state);
    } else if hit(x, y, 408, 636, 150, 42) {
        return super::sign_both::sign_both(state);
    } else if hit(x, y, 910, 470, 324, 42) {
        return super::broadcast::broadcast(state);
    } else {
        return EventOutcome::Idle;
    }
    EventOutcome::Repaint
}

pub(super) fn proof(state: &mut State, x: u32, y: u32) -> EventOutcome {
    let f = if hit(x, y, 954, 228, 60, 40) {
        0
    } else if hit(x, y, 1014, 228, 110, 40) {
        1
    } else if hit(x, y, 1124, 228, 130, 40) {
        2
    } else {
        return EventOutcome::Idle;
    };
    state.proof_filter = f;
    EventOutcome::Repaint
}

pub(super) fn nox(state: &mut State, x: u32, y: u32) -> EventOutcome {
    if hit(x, y, 246, 438, 200, 36) {
        state.stake_unstake = 0;
    } else if hit(x, y, 446, 438, 200, 36) {
        state.stake_unstake = 1;
    } else if hit(x, y, 246, 516, 560, 30) {
        let rel = x.saturating_sub(246).min(560);
        state.stake_amount = rel * crate::wallet::state::MAX_STAKE / 560;
    } else if hit(x, y, 246, 626, 560, 42) {
        return super::sign_nox::sign_nox(state);
    } else if hit(x, y, 862, 496, 372, 42) {
        return super::probe_tick::probe_kick(state);
    } else {
        return EventOutcome::Idle;
    }
    EventOutcome::Repaint
}
