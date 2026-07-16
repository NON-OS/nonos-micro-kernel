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

// NONOS_NODESKTOP=1 builds bring up drivers and the input stack but leave
// the compositor, splash and shell unspawned, so the on-screen kernel log
// stays visible for the whole session. This is the hardware bring-up
// channel: one photo shows every driver that initialized and where a fault
// lands, with nothing painting over it and no serial cable required.
fn desktop_enabled() -> bool {
    option_env!("NONOS_NODESKTOP").is_none()
}

pub(super) fn spawn() {
    if !desktop_enabled() {
        return;
    }
    spawn_gui_core();
    spawn_boot_splash();
    spawn_wm();
    spawn_wallpaper_catalog();
    spawn_wallpaper();
    spawn_shell();
    super::desktop_services::spawn();
}

// The display core (input router, compositor, boot-splash) is brought up
// before the driver and network fleets so the boot-splash attestation
// screen appears immediately after the loader hands off, and holds while
// the rest of the capsules spawn behind it. `spawn` re-invokes these
// later; every one is idempotent through its `is_alive` guard.
pub(super) fn spawn_early_display() {
    if !desktop_enabled() {
        return;
    }
    spawn_gui_core();
    spawn_boot_splash();
}

pub(super) fn spawn_gui_core() {
    spawn_input_router();
    spawn_compositor();
}

#[cfg(feature = "nonos-capsule-boot-splash")]
fn spawn_boot_splash() {
    use crate::userspace::capsule_boot_splash as c;
    if c::shared_state().is_alive() {
        return;
    }
    super::boot::capsule(
        "BOOT-SPLASH",
        "boot_splash",
        c::spawn_boot_splash_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-boot-splash"))]
fn spawn_boot_splash() {}

#[cfg(feature = "nonos-capsule-wallpaper-catalog")]
fn spawn_wallpaper_catalog() {
    use crate::userspace::capsule_wallpaper_catalog as c;
    super::boot::capsule(
        "WALLPAPER-CATALOG",
        "wallpaper_catalog",
        c::spawn_wallpaper_catalog_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-wallpaper-catalog"))]
fn spawn_wallpaper_catalog() {}

#[cfg(feature = "nonos-capsule-input-router")]
fn spawn_input_router() {
    use crate::userspace::capsule_input_router as c;
    if c::shared_state().is_alive() {
        return;
    }
    super::boot::capsule(
        "INPUT-ROUTER",
        "input_router",
        c::spawn_input_router_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-input-router"))]
fn spawn_input_router() {}

#[cfg(feature = "nonos-capsule-compositor")]
fn spawn_compositor() {
    use crate::userspace::capsule_compositor as c;
    if c::shared_state().is_alive() {
        return;
    }
    super::boot::capsule("COMPOSITOR", "compositor", c::spawn_compositor_capsule, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-compositor"))]
fn spawn_compositor() {}

#[cfg(feature = "nonos-capsule-wm")]
fn spawn_wm() {
    use crate::userspace::capsule_wm as c;
    super::boot::capsule("WM", "wm", c::spawn_wm_capsule, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-wm"))]
fn spawn_wm() {}

#[cfg(feature = "nonos-capsule-wallpaper")]
fn spawn_wallpaper() {
    use crate::userspace::capsule_wallpaper as c;
    super::boot::capsule("WALLPAPER", "wallpaper", c::spawn_wallpaper_capsule, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-wallpaper"))]
fn spawn_wallpaper() {}

#[cfg(feature = "nonos-capsule-desktop-shell")]
fn spawn_shell() {
    use crate::userspace::capsule_desktop_shell as c;
    super::boot::capsule(
        "DESKTOP-SHELL",
        "desktop_shell",
        c::spawn_desktop_shell_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-desktop-shell"))]
fn spawn_shell() {}
