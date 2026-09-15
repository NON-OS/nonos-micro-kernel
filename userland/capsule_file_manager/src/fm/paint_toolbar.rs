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

use nonos_app_skeleton::PaintBuffer;

use super::chrome_pill::PillState;
use super::header_bar::clear_slot;
use super::header_layout::{sort_text, tool_slots, tool_y};
use super::header_nav::{can_back, can_fwd, nav_slots};
use super::header_slots::{HeadHit, Slot, UNDO_LABEL};
use super::icon_path::Icon;
use super::paint_tool_pill::pill;
use super::state::{State, ViewKind};
use super::theme::{INK2, INK3};
use super::toolbar_buttons::{group_plate, icon_btn};
use super::toolbar_new::{icon_label, new_btn};
use super::toolbar_search::search;

pub fn paint_toolbar(state: &State, fb: &mut PaintBuffer) {
    let y = tool_y();
    paint_nav(state, fb, y);
    let slots = tool_slots(state);
    toggle_ground(fb, &slots, y);
    let sort = sort_text(state);
    let clear = clear_slot(state, &slots);
    for slot in &slots {
        match slot.hit {
            HeadHit::Search => search(state, fb, slot, y, clear.as_ref()),
            HeadHit::ViewGrid => {
                icon_btn(fb, slot, y, Icon::Grid, state.view == ViewKind::Grid, false)
            }
            HeadHit::ViewList => {
                icon_btn(fb, slot, y, Icon::List, state.view == ViewKind::List, false)
            }
            HeadHit::Sort => icon_label(fb, slot, y, Icon::Sliders, &sort, PillState::Idle),
            HeadHit::Undo => pill(fb, slot, y, UNDO_LABEL, undo_ink(state)),
            HeadHit::New => new_btn(fb, slot, y),
            _ => {}
        }
    }
}

// The history group. Forward has no visit stack behind it, so it draws dimmed
// and stays inert rather than looking live and doing nothing.
fn paint_nav(state: &State, fb: &mut PaintBuffer, y: u32) {
    let slots = nav_slots();
    let (Some(first), Some(last)) = (slots.first(), slots.last()) else { return };
    group_plate(fb, first, last, y);
    for slot in &slots {
        let back = slot.hit == HeadHit::NavBack;
        let live = if back { can_back(state) } else { can_fwd(state) };
        let icon = if back { Icon::Back } else { Icon::Forward };
        icon_btn(fb, slot, y, icon, false, !live);
    }
}

// Undo dims to the tertiary ink when the stack holds nothing to replay.
fn undo_ink(state: &State) -> u32 {
    if state.undo.is_empty() {
        INK3
    } else {
        INK2
    }
}

// The two halves share one ground, so the toggle reads as a single track with a
// lit active half rather than as two adjacent pills.
fn toggle_ground(fb: &mut PaintBuffer, slots: &[Slot], y: u32) {
    let grid = slots.iter().find(|s| s.hit == HeadHit::ViewGrid);
    let list = slots.iter().find(|s| s.hit == HeadHit::ViewList);
    if let (Some(grid), Some(list)) = (grid, list) {
        group_plate(fb, grid, list, y);
    }
}
