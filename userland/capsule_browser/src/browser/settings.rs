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

//! The settings panel behind the menu button. It shows the SOCKS5 proxy state
//! and offers two actions: start setting a proxy (which focuses the address bar
//! with the command prefix, reusing the address input) and turn the proxy off.

use alloc::format;
use alloc::string::String;

use nonos_app_skeleton::{EventOutcome, PaintBuffer};

use crate::browser::manifest::WIDTH;
use crate::browser::paint::chrome::constants;
use crate::browser::state::State;

const PANEL_W: i32 = 300;
const PANEL_H: i32 = 150;
const PANEL_TOP: i32 = constants::TITLEBAR as i32 + 56;
const SET_Y: i32 = PANEL_TOP + 74;
const OFF_Y: i32 = PANEL_TOP + 110;
const ROW_H: i32 = 30;

fn panel_x(width: i32) -> i32 {
    width - PANEL_W - 12
}

fn width_of(state: &State) -> i32 {
    if state.viewport_w > 0 {
        state.viewport_w as i32
    } else {
        WIDTH as i32
    }
}

/// Draw the panel over the page when it is open.
pub fn paint(state: &State, fb: &mut PaintBuffer) {
    if !state.settings_open {
        return;
    }
    let x = panel_x(fb.width as i32);
    fb.fill_rect(x as u32, PANEL_TOP as u32, PANEL_W as u32, PANEL_H as u32, constants::TOOLBAR_BG);
    fb.fill_rect(x as u32, PANEL_TOP as u32, PANEL_W as u32, 2, constants::ACCENT);
    fb.text_ttf(x + 14, PANEL_TOP + 12, "Settings", constants::FG, 16.0);

    let status = match state.proxy.as_ref() {
        Some(p) => format!("SOCKS5 proxy: {}:{}", p.host, p.port),
        None => String::from("SOCKS5 proxy: off"),
    };
    fb.text_ttf(x + 14, PANEL_TOP + 42, &status, constants::DIM, 14.0);

    button(fb, x, SET_Y, "Set proxy (type host:port)");
    button(fb, x, OFF_Y, "Turn proxy off");
}

fn button(fb: &mut PaintBuffer, x: i32, y: i32, label: &str) {
    fb.fill_rect(
        (x + 12) as u32,
        y as u32,
        (PANEL_W - 24) as u32,
        ROW_H as u32,
        constants::FIELD_BG,
    );
    fb.text_ttf(x + 24, y + 8, label, constants::FG, 14.0);
}

enum Action {
    Set,
    Off,
    Close,
    Ignore,
}

fn action_at(x: i32, y: i32, width: i32) -> Action {
    let px = panel_x(width);
    let inside = x >= px && x < px + PANEL_W && y >= PANEL_TOP && y < PANEL_TOP + PANEL_H;
    if !inside {
        return Action::Close;
    }
    if y >= SET_Y && y < SET_Y + ROW_H {
        return Action::Set;
    }
    if y >= OFF_Y && y < OFF_Y + ROW_H {
        return Action::Off;
    }
    Action::Ignore
}

/// Handle a click while the panel is open. A click on empty space, or anywhere
/// outside the panel, closes it. "Set proxy" focuses the address bar with the
/// `proxy socks5://` prefix so the user types the host:port and presses Enter,
/// which the existing address command applies. "Turn proxy off" clears it.
pub fn on_click(state: &mut State, x: i32, y: i32) -> EventOutcome {
    match action_at(x, y, width_of(state)) {
        Action::Set => {
            state.settings_open = false;
            state.address = String::from("proxy socks5://");
            state.address_focused = true;
            state.status = String::from("type host:port then press Enter");
        }
        Action::Off => {
            state.proxy = None;
            state.settings_open = false;
            state.status = String::from("proxy off");
        }
        Action::Close => {
            state.settings_open = false;
        }
        Action::Ignore => {}
    }
    EventOutcome::Repaint
}
