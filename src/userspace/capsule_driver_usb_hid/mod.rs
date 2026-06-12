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

pub mod client;
mod embed;
mod error;
mod protocol;
mod spawn;
mod state;

pub use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;
pub use client::{
    feed_keyboard_report, feed_mouse_report, get_state, healthcheck, poll_keys, poll_mouse,
    probe_config, HidBinding, HidKind, KeyEvent, MouseEvent, UsbHidState,
};
pub use error::UsbHidError;
pub use spawn::spawn_driver_usb_hid_capsule;
pub use state::shared_state;
