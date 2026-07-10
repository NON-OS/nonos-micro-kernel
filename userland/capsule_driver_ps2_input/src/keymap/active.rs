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

//! The active keyboard layout, cycled at runtime with Ctrl+Alt+Space.
//! Nothing downstream resolves shift or layout (apps insert the event
//! code as the final character), so the driver owns this state; a
//! settings-service hook can select it explicitly later.

use core::sync::atomic::{AtomicU8, Ordering};

use nonos_keymap::Layout;

static LAYOUT_INDEX: AtomicU8 = AtomicU8::new(0);

pub fn current() -> Layout {
    Layout::from_index(LAYOUT_INDEX.load(Ordering::Relaxed))
}

/// Advance to the next layout and announce it on the debug channel
/// (best effort; the line only appears when the build grants the driver
/// serial debug). Returns the new layout.
pub fn cycle() -> Layout {
    let next = current().next();
    LAYOUT_INDEX.store(next.index(), Ordering::Relaxed);
    let mut msg = *b"ps2: keyboard layout ??\n";
    let name = next.name();
    msg[21] = name[0];
    msg[22] = name[1];
    let _ = nonos_libc::mk_debug(msg.as_ptr(), msg.len());
    next
}
