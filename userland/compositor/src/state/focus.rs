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

// Compositor-side focus tracker. The wm capsule owns z-order and
// window state; this just records who currently owns input dispatch
// so input_router can be queried at any point without a round trip
// through wm.

pub struct FocusTable {
    focused_pid: u32,
}

impl FocusTable {
    pub const fn new() -> Self {
        Self { focused_pid: 0 }
    }

    // Set the focused process, returning true if it actually changed. A change
    // reorders the draw stack, so the caller must fully recomposite; otherwise
    // the overlap between the old and new top window keeps stale pixels that a
    // later partial repaint smears as the cursor passes over them.
    pub fn set(&mut self, pid: u32) -> bool {
        if self.focused_pid == pid {
            return false;
        }
        self.focused_pid = pid;
        true
    }

    // The process that currently owns focus, or 0 for none. The compositor draws
    // this owner's window on top within its z-band, so a click raises it.
    pub fn focused(&self) -> u32 {
        self.focused_pid
    }
}
