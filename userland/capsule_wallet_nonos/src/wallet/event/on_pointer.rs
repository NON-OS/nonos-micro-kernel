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

use crate::wallet::state::{State, SEND_FIELD_AMOUNT, SEND_FIELD_NONCE, SEND_FIELD_TO, VIEW_HOME, VIEW_PROOF, VIEW_RECEIVE, VIEW_SEND};

pub fn on_pointer(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if x < 0 || y < 0 {
        return EventOutcome::Idle;
    }
    let x = x as u32;
    let y = y as u32;
    if nav(state, x, y) {
        return EventOutcome::Repaint;
    }
    if hit(x, y, 368, 300, 180, 42) {
        return super::generate::generate(state);
    }
    if hit(x, y, 760, 300, 170, 42) {
        return super::probe_net::probe_net(state);
    }
    if state.view == VIEW_SEND {
        return send(state, x, y);
    }
    EventOutcome::Idle
}

fn nav(state: &mut State, x: u32, y: u32) -> bool {
    let view = if hit(x, y, 32, 160, 220, 38) { VIEW_HOME } else if hit(x, y, 32, 212, 220, 38) { VIEW_RECEIVE } else if hit(x, y, 32, 264, 220, 38) { VIEW_SEND } else if hit(x, y, 32, 316, 220, 38) { VIEW_PROOF } else { 255 };
    if view == 255 {
        return false;
    }
    state.view = view;
    true
}

fn send(state: &mut State, x: u32, y: u32) -> EventOutcome {
    if hit(x, y, 368, 214, 520, 58) {
        state.send_focus = SEND_FIELD_TO;
    } else if hit(x, y, 368, 296, 520, 58) {
        state.send_focus = SEND_FIELD_AMOUNT;
    } else if hit(x, y, 368, 378, 520, 58) {
        state.send_focus = SEND_FIELD_NONCE;
    } else if hit(x, y, 368, 474, 210, 42) {
        return super::sign_eth::sign_eth(state);
    } else {
        return EventOutcome::Idle;
    }
    EventOutcome::Repaint
}

fn hit(x: u32, y: u32, hx: u32, hy: u32, hw: u32, hh: u32) -> bool {
    x >= hx && y >= hy && x < hx + hw && y < hy + hh
}
