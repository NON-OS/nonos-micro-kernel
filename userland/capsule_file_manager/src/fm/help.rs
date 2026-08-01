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

use nonos_app_skeleton::{EventOutcome, InputEvent, PaintBuffer};

use super::manifest::HEIGHT;
use super::state::{Mode, State};
use super::theme::{BACKGROUND, FOREGROUND, MUTED};

const LEFT: u32 = 16;
const FIRST_Y: u32 = 44;
const LINE_H: u32 = 18;

const KEYS: [&str; 17] = [
    "arrows / j k h l   move and open",
    "Enter / l          open dir or preview file",
    "Backspace / h      up a directory",
    "space              check / uncheck entry",
    "a                  select all in view",
    "n                  new file",
    "m                  new directory",
    "r                  rename",
    "d                  delete (selection or cursor)",
    "c / x              copy / cut",
    "p                  paste into current dir",
    "o                  duplicate",
    "u                  toggle read-only",
    "s                  cycle sort (name/size/date/type)",
    "wheel              scroll the listing",
    "/                  filter, type to search",
    "? / esc            toggle this help",
];

// Full-window keybind reference.
pub fn paint_help(fb: &mut PaintBuffer) {
    fb.clear(BACKGROUND);
    fb.text(LEFT, 18, b"file_manager keys", FOREGROUND);
    let mut y = FIRST_Y;
    for line in KEYS {
        fb.text(LEFT, y, line.as_bytes(), MUTED);
        y += LINE_H;
    }
    fb.text(LEFT, HEIGHT - 22, b"any key to close", MUTED);
}

// Any key dismisses the help back to browsing.
pub fn on_key(state: &mut State, _event: InputEvent) -> EventOutcome {
    state.mode = Mode::Browse;
    state.status = b"click or Enter to open";
    EventOutcome::Repaint
}
