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

use super::{NotifyLevel, PkgInstallPrompt, SpotlightState, TaskbarState, ToastQueue, TrayTable};

pub struct Context {
    pub compositor_port: u32,
    pub wm_port: u32,
    pub input_router_port: u32,
    pub input_kind_mask: u32,
    pub input_ready: bool,
    pub wm_notify_ready: bool,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub backing_va: u64,
    pub tray: TrayTable,
    pub taskbar: TaskbarState,
    pub spotlight: SpotlightState,
    /// Whether the full-screen Launchpad overlay is open.
    pub launchpad: bool,
    pub last_notify_level: Option<NotifyLevel>,
    pub toasts: ToastQueue,
    pub toast_layer_live: bool,
    pub net_was_online: bool,
    pub clock_24h: bool,
    pub policy_port: u32,
    pub next_request_id: u32,
    /// Entries at the VFS root, shown as icons on the desktop. Loaded lazily
    /// once the vfs_pool service is up, then refreshed after we mutate it.
    pub desktop_items: alloc::vec::Vec<crate::vfs_client::Entry>,
    /// Base names of the capsule-store apps the installer reports, already
    /// filtered of anything impersonating a built-in. Loaded lazily once the
    /// installer service is up, then left alone.
    pub installed_apps: alloc::vec::Vec<alloc::vec::Vec<u8>>,
    pub installed_apps_loaded: bool,
    /// File names of the `.nonos` packages sitting in /pkgs, rescanned every
    /// tick so one dropped in after boot appears without a restart.
    pub pkg_files: alloc::vec::Vec<alloc::string::String>,
    pub pkg_files_loaded: bool,
    /// Pid the installer handed back for each store app we have launched, so a
    /// second click focuses that window instead of loading another copy. An
    /// entry is dropped once its pid stops accepting control frames.
    pub installed_pids: alloc::collections::BTreeMap<alloc::vec::Vec<u8>, u32>,
    /// Top-left corner of the desktop right-click menu, or None when hidden.
    pub desktop_menu: Option<(u32, u32)>,
    /// Which menu row the pointer is over, so it can be highlighted.
    pub menu_hover: Option<usize>,
    /// The desktop item the open menu acts on; None means the empty-desktop
    /// menu (New Folder / New File) rather than the per-item one.
    pub menu_target: Option<usize>,
    /// The item being renamed and its edit buffer, while an inline rename is in
    /// progress.
    pub rename: Option<usize>,
    pub rename_buf: alloc::string::String,
    /// Drag-to-move: the icon a left-press started on, whether the pointer has
    /// moved far enough to count as a drag, and the current pointer position so
    /// the dragged icon can follow the cursor.
    pub drag_from: Option<usize>,
    pub drag_moved: bool,
    pub drag_x: u32,
    pub drag_y: u32,
    /// Open-with broker: path handed to a target app's service name by
    /// `OP_OPEN_WITH`, pulled once via `OP_TAKE_OPEN_ARG` after the app wakes.
    pub pending_open: alloc::collections::BTreeMap<alloc::string::String, alloc::string::String>,
    /// Name of the runtime-installed app whose launch is awaiting the consent
    /// modal, or None when no dialog is up.
    pub pending_consent: Option<alloc::vec::Vec<u8>>,
    /// The /pkgs package whose install awaits the consent modal, carrying the
    /// summary pkg_query returned, or None when no dialog is up.
    pub pending_pkg_install: Option<PkgInstallPrompt>,
}

impl Context {
    pub fn issue_request_id(&mut self) -> u32 {
        let id = self.next_request_id;
        self.next_request_id = id.wrapping_add(1).max(1);
        id
    }
}
