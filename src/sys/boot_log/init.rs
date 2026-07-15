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

use super::state::{DISPLAY_ENABLED, LOG_Y, MIN_LOG_Y};
use core::sync::atomic::Ordering;

const TOP: u32 = 24;

// The on-screen kernel text console stays off by design: the bootloader
// leaves its verified-boot splash in the framebuffer and the compositor
// paints the desktop over it. NONOS_FBCONSOLE=1 at build time flips it on
// for hardware bring-up, where the panel is the only console there is and
// every kernel milestone must be photographable.
const FBCONSOLE: Option<&'static str> = option_env!("NONOS_FBCONSOLE");

pub fn init_after_fb(_cursor_y: u32) {
    LOG_Y.store(TOP, Ordering::SeqCst);
    MIN_LOG_Y.store(TOP, Ordering::SeqCst);
    if matches!(FBCONSOLE, Some("1")) {
        crate::sys::serial::println(b"[fbconsole] on-screen log enabled (bring-up build)");
        DISPLAY_ENABLED.store(true, Ordering::Release);
        return;
    }
    crate::sys::serial::println(b"[fbconsole] on-screen log disabled; serial only");
    DISPLAY_ENABLED.store(false, Ordering::Release);
}

pub fn disable_display() {
    DISPLAY_ENABLED.store(false, Ordering::Release);
}
