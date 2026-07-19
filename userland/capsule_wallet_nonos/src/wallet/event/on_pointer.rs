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

use crate::wallet::state::{State, SEND_FIELD_AMOUNT, SEND_FIELD_TO, VIEW_HOME, VIEW_NOX, VIEW_PROOF, VIEW_RECEIVE, VIEW_SEND, VIEW_SHIELDED};

pub fn on_pointer(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if x < 0 || y < 0 {
        return EventOutcome::Idle;
    }
    let x = x as u32;
    let y = y as u32;
    if nav(state, x, y) {
        return EventOutcome::Repaint;
    }
    use crate::wallet::paint::{GEN_BTN_H, GEN_BTN_W, GEN_BTN_X, GEN_BTN_Y};
    if state.view == VIEW_RECEIVE && hit(x, y, GEN_BTN_X, GEN_BTN_Y, GEN_BTN_W, GEN_BTN_H) {
        return if state.address_ready {
            super::probe_net::probe_net(state)
        } else {
            super::generate::generate(state)
        };
    }
    if state.view == VIEW_SEND {
        return send(state, x, y);
    }
    EventOutcome::Idle
}

// Sidebar nav: derive the item index from the same NAV_* geometry the sidebar
// paints with, so drawing and click-mapping can never drift apart.
fn nav(state: &mut State, x: u32, y: u32) -> bool {
    use crate::wallet::paint::{NAV_H, NAV_STEP, NAV_W, NAV_X, NAV_Y0};
    if x < NAV_X || x >= NAV_X + NAV_W || y < NAV_Y0 {
        return false;
    }
    let rel = y - NAV_Y0;
    let i = rel / NAV_STEP;
    if i >= 6 || rel % NAV_STEP >= NAV_H {
        return false;
    }
    let views = [VIEW_HOME, VIEW_RECEIVE, VIEW_SEND, VIEW_PROOF, VIEW_SHIELDED, VIEW_NOX];
    state.view = views[i as usize];
    true
}

fn send(state: &mut State, x: u32, y: u32) -> EventOutcome {
    if hit(x, y, 246, 182, 600, 40) {
        state.send_focus = SEND_FIELD_TO;
    } else if hit(x, y, 246, 342, 508, 40) {
        state.send_focus = SEND_FIELD_AMOUNT;
    } else if hit(x, y, 246, 636, 150, 42) {
        return super::sign_eth::sign_eth(state);
    } else {
        return EventOutcome::Idle;
    }
    EventOutcome::Repaint
}

fn hit(x: u32, y: u32, hx: u32, hy: u32, hw: u32, hh: u32) -> bool {
    x >= hx && y >= hy && x < hx + hw && y < hy + hh
}
