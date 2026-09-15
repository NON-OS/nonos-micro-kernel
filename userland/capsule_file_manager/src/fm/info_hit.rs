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

use super::header_slots::TOOL_H;
use super::info_act::{remove_tag, run_quick};
use super::info_geom::{info_geom, InfoGeom};
use super::info_perms::perm_btn_w;
use super::info_quick::quick_slots;
use super::info_sizes::QUICK_H;
use super::info_tag_hit::{tag_aim, TagAim};
use super::layout::info_x;
use super::perms::toggle_readonly;
use super::prompt_start::start_prompt;
use super::screen::Screen;
use super::state::{PromptKind, State};

/// The info strip owns its whole column. Returning `Some` for any click inside
/// it is what stops a click on the panel from falling through to the row
/// hit-test, which resolves on `y` alone and would select an unrelated row.
pub fn info_click(state: &mut State, x: u32, y: u32) -> Option<EventOutcome> {
    if state.screen != Screen::Browse || x < info_x(state.win_w) {
        return None;
    }
    let Some(g) = info_geom(state, state.win_w, state.win_h) else {
        return Some(EventOutcome::Idle);
    };
    Some(control(state, &g, x, y))
}

fn control(state: &mut State, g: &InfoGeom, x: u32, y: u32) -> EventOutcome {
    if y >= g.quick_y && y < g.quick_y + QUICK_H && g.quick_y + QUICK_H <= g.bottom {
        if let Some(slot) = quick_slots(g.x, g.w).into_iter().find(|s| x >= s.x && x < s.x + s.w) {
            return run_quick(state, slot.action);
        }
    }
    match tag_aim(state, g, x, y) {
        Some(TagAim::Remove(name)) => return remove_tag(state, name.as_str()),
        Some(TagAim::Add) => return start_prompt(state, PromptKind::Tag, b"tag: "),
        None => {}
    }
    if perms_hit(state, g, x, y) {
        toggle_readonly(state);
        return EventOutcome::Repaint;
    }
    EventOutcome::Idle
}

fn perms_hit(state: &State, g: &InfoGeom, x: u32, y: u32) -> bool {
    let Some(entry) = state.entries.get(state.cursor) else { return false };
    let w = perm_btn_w(entry.writable).min(g.w);
    g.perms_btn + TOOL_H <= g.bottom
        && y >= g.perms_btn
        && y < g.perms_btn + TOOL_H
        && x >= g.x
        && x < g.x + w
}
