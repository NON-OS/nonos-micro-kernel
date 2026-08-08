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
    // The backup screen owns input until the phrase is confirmed written
    // down; the confirm button is the only live control.
    if state.backup_active {
        if hit(x, y, 246, 560, 280, 42) {
            return super::backup::confirm_backup(state);
        }
        return EventOutcome::Idle;
    }
    // The import and recovery fields own input while open; keyboard-driven.
    if state.import_active || state.recover_active {
        return EventOutcome::Idle;
    }
    // Export / Hide toggle in the account card (rx = 542, card at y = 384).
    // Height matches the 42px the outline button actually draws.
    if state.address_ready && hit(x, y, 842, 392, 96, 42) {
        return super::export_key::toggle_export(state);
    }
    // Set-up panel: Generate a fresh HD account, import a raw key, or recover
    // from a phrase.
    if hit(x, y, 562, 318, 150, 42) {
        return super::generate::generate(state);
    }
    if hit(x, y, 724, 318, 180, 42) {
        return super::import::toggle_import(state);
    }
    if hit(x, y, 916, 318, 160, 42) {
        return super::recover::toggle_recover(state);
    }
    EventOutcome::Idle
}

pub(super) fn send(state: &mut State, x: u32, y: u32) -> EventOutcome {
    // Coordinates match paint_send.rs: card at cx=226, fields inset at ix=246,
    // iw=600. The ETH/NOX asset tabs sit at ix+iw-156 and ix+iw-76 (690, 770).
    if hit(x, y, 690, 152, 74, 26) {
        state.send_token = 0;
    } else if hit(x, y, 770, 152, 74, 26) {
        state.send_token = 1;
    } else if hit(x, y, 246, 182, 600, 40) {
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

pub(super) fn proof(_state: &mut State, _x: u32, _y: u32) -> EventOutcome {
    // The proof view is a read-only record of the signed transaction; nothing
    // to click.
    EventOutcome::Idle
}

pub(super) fn nox(state: &mut State, x: u32, y: u32) -> EventOutcome {
    // Geometry comes from the same layout the painter used, so a resized
    // window cannot draw a control in one place and answer it in another.
    let l = crate::wallet::paint::NoxLayout::sized(state.view_w, state.view_h);
    let tabs = l.tabs_y();
    if hit(x, y, l.cx + 20, tabs, l.tab_w, 36) {
        state.stake_unstake = 0;
    } else if hit(x, y, l.cx + 20 + l.tab_w, tabs, l.tab_w, 36) {
        state.stake_unstake = 1;
    } else if state.stake_unstake == 0 && hit(x, y, l.track_x, l.track_y() - 8, l.track_w, 26) {
        // The bar spans what this wallet holds, so its far end is the whole
        // balance rather than a ceiling nobody has. Typing is what reaches an
        // exact figure; the bar is for choosing a proportion of it. Dragging
        // to the very end sets the balance exactly, since the arithmetic that
        // gets there otherwise loses the last wei to the division.
        let rel = x.saturating_sub(l.track_x).min(l.track_w) as u128;
        let held = crate::wallet::nox::held_wei(state.nox.balance_ready, &state.nox.balance_wei)
            .unwrap_or(0);
        let span = l.track_w.max(1) as u128;
        let wei = if rel >= span { held } else { held / span * rel };
        super::stake_set::set_wei(state, wei);
    } else if let Some(term) = lock_chip(&l, x, y) {
        state.stake_lock = term;
    } else if hit(x, y, l.cx + 20, l.action_y(), l.track_w, 42) {
        return super::stake_flow::stake_flow(state);
    } else {
        return EventOutcome::Idle;
    }
    EventOutcome::Repaint
}

// Which lock chip a click landed on, if any. Chips share the lock card's
// width evenly, exactly as `nox_lock` draws them.
fn lock_chip(l: &crate::wallet::paint::NoxLayout, x: u32, y: u32) -> Option<u8> {
    let ly = l.lock_y();
    let terms = crate::wallet::nox::LOCK_TERMS.len() as u32;
    let chips = l.rw.saturating_sub(40) / terms;
    if chips == 0 || !hit(x, y, l.rx + 20, ly + 32, chips * terms, 26) {
        return None;
    }
    Some((x.saturating_sub(l.rx + 20) / chips).min(terms - 1) as u8)
}
