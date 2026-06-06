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
mod config_set_max_slots;
mod crcr_program;
mod dcbaap_program;
mod portsc_clear_changes;
mod portsc_read;
mod portsc_write;
mod usbcmd_read;
mod usbcmd_write;
mod usbsts_clear;
mod usbsts_read;
pub use config_set_max_slots::config_set_max_slots;
pub use crcr_program::crcr_program;
pub use dcbaap_program::dcbaap_program;
pub use portsc_clear_changes::portsc_clear_changes;
pub use portsc_read::portsc_read;
pub use portsc_write::portsc_write;
pub use usbcmd_read::usbcmd_read;
pub use usbcmd_write::usbcmd_write;
pub use usbsts_clear::usbsts_clear;
pub use usbsts_read::usbsts_read;
