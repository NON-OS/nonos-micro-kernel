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

use super::enable_port::enable_port;
use crate::init::enable_scanning;

// Bring the keyboard online without resetting the controller: the firmware
// already initialized the i8042 (the machine boots and types in its own BIOS).
// A keyboard RESET (0xFF) makes the device reply with a delayed self-test byte
// that lands after init returns and desyncs the scancode stream, and rewriting
// the config byte off a possibly-stale read can silence a working controller,
// so neither is done here. What is required: explicitly enable the first port
// (firmware may hand off with it disabled) and then start scanning.
pub fn enable_keyboard(grant_id: u64) -> Result<(), &'static str> {
    enable_port(grant_id)?;
    enable_scanning(grant_id)
}
