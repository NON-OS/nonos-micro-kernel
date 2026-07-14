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

use super::desktop_enabled::desktop_enabled;
use super::spawn_boot_splash::spawn_boot_splash;
use super::spawn_gui_core::spawn_gui_core;

// The display core (input router, compositor, boot-splash) is brought up
// before the driver and network fleets so the boot-splash attestation
// screen appears immediately after the loader hands off, and holds while
// the rest of the capsules spawn behind it. `spawn` re-invokes these
// later; every one is idempotent through its `is_alive` guard.
pub(crate) fn spawn_early_display() {
    if !desktop_enabled() {
        return;
    }
    spawn_gui_core();
    spawn_boot_splash();
}
