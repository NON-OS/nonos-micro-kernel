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

use nonos_toolkit::decorations::{hit_test, margin, DecorationHit};

use crate::input::{InputEvent, InputKind};

const MENUBAR_H: i64 = 28;
const EDGE: u32 = 10; // grab band on the right / bottom borders
const MIN_W: u32 = 300;
const MIN_H: u32 = 200;
const STEP: u32 = 8; // only resize once the drag moves this far (throttle reallocs)

pub(super) struct DragState {
    pub active: bool,
    pub hover: DecorationHit,
    press_x: i32,
    press_y: i32,
    base_x: u32,
    base_y: u32,
    resizing: bool,
    rz_right: bool,
    rz_bottom: bool,
    last_w: u32,
    last_h: u32,
}

impl DragState {
    pub(super) const fn new() -> Self {
        Self {
            active: false,
            hover: DecorationHit::None,
            press_x: 0,
            press_y: 0,
            base_x: 0,
            base_y: 0,
            resizing: false,
            rz_right: false,
            rz_bottom: false,
            last_w: 0,
            last_h: 0,
        }
    }
}

pub(super) enum PointerAction {
    None,
    MoveTo(u32, u32),
    ResizeTo(u32, u32),
    HoverChanged,
}

pub(super) fn handle(
    state: &mut DragState,
    width: u32,
    height: u32,
    win_x: u32,
    win_y: u32,
    maximized: bool,
    event: &InputEvent,
) -> PointerAction {
    let m = margin(maximized);
    match event.kind {
        InputKind::ButtonDown => {
            if event.x >= 0 && event.y >= 0 {
                let (x, y) = (event.x as u32, event.y as u32);
                let on_right = x + EDGE + m >= width;
                let on_bottom = y + EDGE + m >= height;
                // Resize grabs the right/bottom borders (below the titlebar);
                // the titlebar itself still moves the window.
                if (on_right || on_bottom) && (event.y as i64) >= MENUBAR_H {
                    state.resizing = true;
                    state.rz_right = on_right;
                    state.rz_bottom = on_bottom;
                    state.last_w = width;
                    state.last_h = height;
                    state.active = false;
                    return PointerAction::None;
                }
                if hit_test(width, height, maximized, x, y) == DecorationHit::Titlebar {
                    state.active = true;
                    state.resizing = false;
                    state.press_x = event.x;
                    state.press_y = event.y;
                    state.base_x = win_x;
                    state.base_y = win_y;
                }
            }
            PointerAction::None
        }
        InputKind::ButtonUp => {
            state.active = false;
            // Commit the resize once, on release, so the surface is reallocated
            // a single time instead of on every drag step (which blinked).
            if state.resizing {
                state.resizing = false;
                if state.last_w >= MIN_W && state.last_h >= MIN_H {
                    return PointerAction::ResizeTo(state.last_w, state.last_h);
                }
            }
            PointerAction::None
        }
        InputKind::PointerAbs if state.resizing => {
            // Track the target size during the drag; do NOT reallocate yet.
            let nw = if state.rz_right { (event.x.max(0) as u32 + m).max(MIN_W) } else { width };
            let nh = if state.rz_bottom { (event.y.max(0) as u32 + m).max(MIN_H) } else { height };
            state.last_w = nw;
            state.last_h = nh;
            let _ = STEP;
            PointerAction::None
        }
        InputKind::PointerAbs if state.active => {
            let nx = state.base_x as i64 + (event.x - state.press_x) as i64;
            let ny = state.base_y as i64 + (event.y - state.press_y) as i64;
            PointerAction::MoveTo(nx.max(0) as u32, ny.max(MENUBAR_H) as u32)
        }
        InputKind::PointerAbs => {
            let hit = if event.x >= 0 && event.y >= 0 {
                hit_test(width, height, maximized, event.x as u32, event.y as u32)
            } else {
                DecorationHit::None
            };
            if hit == state.hover {
                return PointerAction::None;
            }
            let was = state.hover;
            state.hover = hit;
            if is_button(hit) || is_button(was) {
                PointerAction::HoverChanged
            } else {
                PointerAction::None
            }
        }
        _ => PointerAction::None,
    }
}

fn is_button(hit: DecorationHit) -> bool {
    matches!(
        hit,
        DecorationHit::CloseButton | DecorationHit::MinimizeButton | DecorationHit::MaximizeButton
    )
}
