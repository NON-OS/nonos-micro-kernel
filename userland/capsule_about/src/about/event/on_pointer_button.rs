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

use crate::about::section::SECTIONS;
use crate::about::state::State;
use crate::about::theme::{
    HEADER_HEIGHT, TAB_BAR_HEIGHT, TAB_HORIZONTAL_PADDING,
};

const TAB_CELL_WIDTH: u32 = 8;

pub fn on_pointer_button(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if x < 0 || y < 0 {
        return EventOutcome::Idle;
    }
    let x = x as u32;
    let y = y as u32;
    let tab_top = HEADER_HEIGHT;
    let tab_bottom = HEADER_HEIGHT.saturating_add(TAB_BAR_HEIGHT);
    if y < tab_top || y >= tab_bottom {
        return EventOutcome::Idle;
    }
    let mut cursor = TAB_HORIZONTAL_PADDING;
    for section in SECTIONS {
        let label_w = section.title().len() as u32 * TAB_CELL_WIDTH;
        let tab_w = label_w.saturating_add(TAB_HORIZONTAL_PADDING * 2);
        let end = cursor.saturating_add(tab_w);
        if x >= cursor && x < end {
            if state.section == section {
                return EventOutcome::Idle;
            }
            state.section = section;
            state.scroll = 0;
            return EventOutcome::Repaint;
        }
        cursor = end;
    }
    EventOutcome::Idle
}
