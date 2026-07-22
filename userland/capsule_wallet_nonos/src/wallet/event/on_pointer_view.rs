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
    // Mirror paint_home.rs: four quick-action cards from cx, each qw wide with a
    // 16px gap, where qw is derived from the live framebuffer width.
    let cx = 226u32;
    let cw = state.view_w.saturating_sub(252);
    let qw = (cw.saturating_sub(48)) / 4;
    if y < 386 || y >= 386 + 82 {
        return EventOutcome::Idle;
    }
    let step = qw + 16;
    let views = [VIEW_SEND, VIEW_RECEIVE, VIEW_NOX, VIEW_NOX];
    for (i, v) in views.iter().enumerate() {
        let bx = cx + i as u32 * step;
        if x >= bx && x < bx + qw {
            state.view = *v;
            return EventOutcome::Repaint;
        }
    }
    EventOutcome::Idle
}

pub(super) fn receive(state: &mut State, x: u32, y: u32) -> EventOutcome {
    // The import field owns input while open; it is driven from the keyboard.
    if state.import_active {
        return EventOutcome::Idle;
    }
    // Export / Hide toggle in the account card (rx = 542, card at y = 384).
    if state.address_ready && hit(x, y, 842, 392, 96, 34) {
        return super::export_key::toggle_export(state);
    }
    // Set-up panel: Generate a fresh account, or open the import field.
    if hit(x, y, 562, 320, 150, 42) {
        return super::generate::generate(state);
    }
    if hit(x, y, 724, 320, 180, 42) {
        return super::import::toggle_import(state);
    }
    EventOutcome::Idle
}

pub(super) fn send(state: &mut State, x: u32, y: u32) -> EventOutcome {
    // Coordinates match paint_send.rs: card at cx=226, fields inset at ix=246.
    if hit(x, y, 246, 182, 600, 40) {
        state.send_focus = SEND_FIELD_TO;
    } else if hit(x, y, 246, 292, 600, 40) {
        state.send_focus = SEND_FIELD_AMOUNT;
    } else if hit(x, y, 246, 500, 150, 42) {
        // "Sign & send": sign the transfer and broadcast it in one press.
        return super::send_now::send_now(state);
    } else {
        return EventOutcome::Idle;
    }
    EventOutcome::Repaint
}

pub(super) fn proof(state: &mut State, x: u32, y: u32) -> EventOutcome {
    // Mirror paint_proof_view.rs seg(): three cells (60/110/130 wide) starting
    // at cx + cw - 300, where cw is the live width less the sidebar.
    let cw = state.view_w.saturating_sub(252);
    let sx = 226 + cw.saturating_sub(300);
    let f = if hit(x, y, sx, 228, 60, 40) {
        0
    } else if hit(x, y, sx + 60, 228, 110, 40) {
        1
    } else if hit(x, y, sx + 170, 228, 130, 40) {
        2
    } else {
        return EventOutcome::Idle;
    };
    state.proof_filter = f;
    EventOutcome::Repaint
}

pub(super) fn nox(state: &mut State, x: u32, y: u32) -> EventOutcome {
    // Coordinates match paint_nox_stake.rs: card at cx=226, controls inset 20.
    if hit(x, y, 246, 438, 200, 36) {
        state.stake_unstake = 0;
    } else if hit(x, y, 446, 438, 200, 36) {
        state.stake_unstake = 1;
    } else if hit(x, y, 246, 520, 560, 26) {
        let rel = x.saturating_sub(246).min(560);
        state.stake_amount = rel * crate::wallet::state::MAX_STAKE / 560;
    } else if hit(x, y, 246, 596, 560, 42) {
        return super::stake_flow::stake_flow(state);
    } else {
        return EventOutcome::Idle;
    }
    EventOutcome::Repaint
}
