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

use super::clipboard::yank;
use super::duplicate::duplicate;
use super::header_slots::TOOL_H;
use super::prompt_start::start_prompt;
use super::sel_geom::{action_y, band};
use super::sel_model::SelAction;
use super::sel_slots::{slots, SelSlot};
use super::selection_clear::clear;
use super::state::{PromptKind, State};

/// A click on the band, tested against the very slots `sel_band` drew from.
/// Returns `None` only when the band is absent or the click missed it, so the
/// caller can fall through to the listing; a click on the band's own ground
/// resolves to `Idle` rather than reaching the row underneath.
pub fn band_click(state: &mut State, x: u32, y: u32) -> Option<EventOutcome> {
    if state.selected.is_empty() {
        return None;
    }
    let b = band(state.win_w, state.win_h);
    if x < b.x || x >= b.x + b.w || y < b.y || y >= b.y + b.h {
        return None;
    }
    let ay = action_y(&b);
    if y < ay || y >= ay + TOOL_H {
        return Some(EventOutcome::Idle);
    }
    match slots(&b).into_iter().find(|s| x >= s.x && x < s.x + s.w) {
        Some(slot) => Some(run(state, &slot)),
        None => Some(EventOutcome::Idle),
    }
}

// Move is a cut awaiting a paste, which is the only move this crate has: the
// vfs client exposes rename within a directory and copy across one, and the
// cut/paste pair is what already drives a cross-directory move.
fn run(state: &mut State, slot: &SelSlot) -> EventOutcome {
    if !slot.wired {
        state.status = b"not available yet";
        return EventOutcome::Repaint;
    }
    match slot.action {
        SelAction::Move => yank(state, true),
        SelAction::Duplicate => duplicate(state),
        SelAction::Tag => return start_prompt(state, PromptKind::Tag, b"tag: "),
        SelAction::Delete => {
            return start_prompt(state, PromptKind::Delete, b"delete? type y + Enter: ")
        }
        SelAction::Clear => {
            clear(state);
            state.status = b"selection cleared";
        }
        SelAction::Share | SelAction::Compress => {}
    }
    EventOutcome::Repaint
}
