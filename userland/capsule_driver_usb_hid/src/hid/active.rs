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

//! The active keyboard layout for USB keyboards, cycled at runtime with
//! Ctrl+Alt+Space. Per-driver state: a USB and a PS/2 keyboard each keep
//! their own layout, which matches per-device expectations until a
//! settings service owns the preference.

use core::sync::atomic::{AtomicU8, Ordering};

use nonos_keymap::Layout;

static LAYOUT_INDEX: AtomicU8 = AtomicU8::new(0);

pub fn current() -> Layout {
    Layout::from_index(LAYOUT_INDEX.load(Ordering::Relaxed))
}

/// Advance to the next layout and announce it on the debug channel (best
/// effort). Returns the new layout.
pub fn cycle() -> Layout {
    let next = current().next();
    LAYOUT_INDEX.store(next.index(), Ordering::Relaxed);
    let mut msg = *b"usb-hid: keyboard layout ??\n";
    let name = next.name();
    msg[25] = name[0];
    msg[26] = name[1];
    let _ = nonos_libc::mk_debug(msg.as_ptr(), msg.len());
    next
}
